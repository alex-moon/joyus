import { DATASTAR_FETCH_EVENT, DSP, DSS } from '@engine/consts'
import { snake } from '@utils/text'
import {getHostFor, getStoreFor} from '@engine/signals'
import type {
  ActionPlugin,
  ActionContext,
  AttributeContext,
  AttributePlugin,
  DatastarFetchEvent,
  HTMLOrSVG,
  Requirement,
  WatcherPlugin,
} from '@engine/types'
import { isHTMLOrSVG } from '@utils/dom'
import { aliasify } from '@utils/text'

const url = 'https://data-star.dev/errors'

const error = (
  ctx: Record<string, any>,
  reason: string,
  metadata: Record<string, any> = {},
) => {
  Object.assign(metadata, ctx)
  const e = new Error()
  const r = snake(reason)
  const q = new URLSearchParams({
    metadata: JSON.stringify(metadata),
  }).toString()
  const c = JSON.stringify(metadata, null, 2)
  e.message = `${reason}\nMore info: ${url}/${r}?${q}\nContext: ${c}`
  return e
}

const actionPlugins: Map<string, ActionPlugin> = new Map()
const attributePlugins: Map<string, AttributePlugin> = new Map()
const watcherPlugins: Map<string, WatcherPlugin> = new Map()

export const actions: Record<
  string,
  (ctx: ActionContext, ...args: any[]) => any
> = new Proxy(
  {},
  {
    get: (_, prop: string) => actionPlugins.get(prop)?.apply,
    has: (_, prop: string) => actionPlugins.has(prop),
    ownKeys: () => Reflect.ownKeys(actionPlugins),
    set: () => false,
    deleteProperty: () => false,
  },
)

// Map of cleanups keyed by element and attribute name
const removals = new Map<HTMLOrSVG, Map<string, () => void>>()

const queuedAttributes: AttributePlugin[] = []
const queuedAttributeNames = new Set<string>()

export const attribute = <R extends Requirement, B extends boolean>(
    plugin: AttributePlugin<R, B>,
): void => {
  // Make the plugin available immediately for any manual apply() calls
  attributePlugins.set(plugin.name, plugin as unknown as AttributePlugin)
  queuedAttributeNames.add(plugin.name)

  // Batch only the auto-apply-once behavior
  queuedAttributes.push(plugin as unknown as AttributePlugin)
  if (queuedAttributes.length === 1) {
    setTimeout(() => {
      queuedAttributes.length = 0
    })
  }
}

export const action = <T>(plugin: ActionPlugin<T>): void => {
  actionPlugins.set(plugin.name, plugin)
}

document.addEventListener(DATASTAR_FETCH_EVENT, ((
  evt: CustomEvent<DatastarFetchEvent>,
) => {
  const plugin = watcherPlugins.get(evt.detail.type)
  if (plugin) {
    plugin.apply(
      {
        el: evt.detail.el as HTMLOrSVG,
        error: error.bind(0, {
          plugin: { type: 'watcher', name: plugin.name },
          element: {
            id: (evt.target as Element).id,
            tag: (evt.target as Element).tagName,
          },
        }),
      },
      evt.detail.argsRaw,
    )
  }
}) as EventListener)

export const watcher = (plugin: WatcherPlugin): void => {
  watcherPlugins.set(plugin.name, plugin)
}

const cleanupEls = (els: Iterable<HTMLOrSVG>): void => {
  for (const el of els) {
    const cleanups = removals.get(el)
    // If removals has el, delete it and run all cleanup functions
    if (removals.delete(el)) {
      for (const cleanup of cleanups!.values()) {
        cleanup()
      }
      cleanups!.clear()
    }
  }
}

const aliasedIgnore = aliasify('ignore')
const aliasedIgnoreAttr = `[${aliasedIgnore}]`
const shouldIgnore = (el: HTMLOrSVG) =>
  el.hasAttribute(`${aliasedIgnore}__self`) || !!el.closest(aliasedIgnoreAttr)

const OBSERVER_CLEANUP_KEY = '__observer__'

const observeRoot = (root: Element | ShadowRoot): void => {
  // Attach cleanup to the "owner" element: the host for ShadowRoot, or the element itself
  const owner = ((root as ShadowRoot).host ?? (root as Element)) as HTMLOrSVG

  let cleanups = removals.get(owner)
  if (cleanups?.has(OBSERVER_CLEANUP_KEY)) {
    return
  }

  const mo = new MutationObserver(observe)
  const opts = { subtree: true, childList: true, attributes: true } as const

  // Observe the tree rooted at `root`
  mo.observe(root, opts)
  // If `root` is a ShadowRoot, also observe host attribute mutations
  if ((root as ShadowRoot).host) {
    mo.observe(owner, { attributes: true })
  }

  if (!cleanups) removals.set(owner, (cleanups = new Map()))
  cleanups.set(OBSERVER_CLEANUP_KEY, () => mo.disconnect())
}

const applyEls = (els: Iterable<HTMLOrSVG>): void => {
  for (const el of els) {
    if (!shouldIgnore(el)) {
      for (const key in el.dataset) {
        applyAttributePlugin(
          el,
          key.replace(/[A-Z]/g, '-$&').toLowerCase(),
          el.dataset[key]!,
        )
      }
    }
  }
}

const observe = (mutations: MutationRecord[]) => {
  for (const {
    target,
    type,
    attributeName,
    addedNodes,
    removedNodes,
  } of mutations) {
    if (type === 'childList') {
      for (const node of removedNodes) {
        if (isHTMLOrSVG(node)) {
          cleanupEls([node])
          cleanupEls(node.querySelectorAll<HTMLOrSVG>('*'))
          const sr = (node as HTMLElement).shadowRoot
          if (sr) {
            cleanupEls(sr.querySelectorAll<HTMLOrSVG>('*'))
          }
        }
      }

      for (const node of addedNodes) {
        if (isHTMLOrSVG(node)) {
          applyEls([node])
          applyEls(node.querySelectorAll<HTMLOrSVG>('*'))
          const sr = (node as HTMLElement).shadowRoot
          if (sr) {
            // Initialize plugins in the shadow subtree
            applyEls(sr.querySelectorAll<HTMLOrSVG>('*'))
            // Start observing the new shadow root so future changes are caught
            observeRoot(sr)
          }
        }
      }
    } else if (
      type === 'attributes' &&
      attributeName!.startsWith('data-') &&
      isHTMLOrSVG(target) &&
      !shouldIgnore(target)
    ) {
      // skip over 'data-'
      const key = attributeName!.slice(5)
      const value = target.getAttribute(attributeName!)
      if (value === null) {
        const cleanups = removals.get(target)
        if (cleanups) {
          cleanups.get(key)?.()
          cleanups.delete(key)
        }
      } else {
        applyAttributePlugin(target, key, value)
      }
    }
  }
}

export const apply = (
  root: HTMLOrSVG | ShadowRoot,
): void => {
  // Apply plugins to the immediate root if it’s an element
  if (isHTMLOrSVG(root)) {
    applyEls([root])
  }

  // Prefer shadowRoot for querying (mirrors your current logic)
  const shadowRoot = (root as HTMLElement).shadowRoot || root;

  // Apply to descendants within the chosen query scope
  applyEls(shadowRoot.querySelectorAll<HTMLOrSVG>('*'))

  // Start observing this scope
  if (shadowRoot !== root) {
    // A host element with a shadow root
    observeRoot(shadowRoot as ShadowRoot)
  } else {
    // A regular element or the document root
    observeRoot(root)
  }
}

const applyAttributePlugin = (
  el: HTMLOrSVG,
  attrKey: string,
  value: string,
): void => {
  if (!ALIAS || attrKey.startsWith(`${ALIAS}-`)) {
    const rawKey = ALIAS ? attrKey.slice(ALIAS.length + 1) : attrKey
    const [namePart, ...rawModifiers] = rawKey.split('__')
    const [pluginName, key] = namePart.split(/:(.+)/)
    const plugin = attributePlugins.get(pluginName)
    if (queuedAttributeNames.has(pluginName) && plugin) {
      const ctx = {
        el,
        rawKey,
        mods: new Map(),
        error: error.bind(0, {
          plugin: { type: 'attribute', name: plugin.name },
          element: { id: el.id, tag: el.tagName },
          expression: { rawKey, key, value },
        }),
        key,
        value,
        rx: undefined,
      } as AttributeContext

      const keyReq =
        (plugin.requirement &&
          (typeof plugin.requirement === 'string'
            ? plugin.requirement
            : plugin.requirement.key)) ||
        'allowed'
      const valueReq =
        (plugin.requirement &&
          (typeof plugin.requirement === 'string'
            ? plugin.requirement
            : plugin.requirement.value)) ||
        'allowed'

      if (key) {
        if (keyReq === 'denied') {
          throw ctx.error('KeyNotAllowed')
        }
      } else if (keyReq === 'must') {
        throw ctx.error('KeyRequired')
      }

      if (value) {
        if (valueReq === 'denied') {
          throw ctx.error('ValueNotAllowed')
        }
      } else if (valueReq === 'must') {
        throw ctx.error('ValueRequired')
      }

      if (keyReq === 'exclusive' || valueReq === 'exclusive') {
        if (key && value) {
          throw ctx.error('KeyAndValueProvided')
        }
        if (!key && !value) {
          throw ctx.error('KeyOrValueRequired')
        }
      }

      if (value) {
        let cachedRx: GenRxFn
        ctx.rx = (...args: any[]) => {
          if (!cachedRx) {
            cachedRx = genRx(value, {
              returnsValue: plugin.returnsValue,
              argNames: plugin.argNames,
            })
          }
          return cachedRx(el, ...args)
        }
      }

      for (const rawMod of rawModifiers) {
        const [label, ...mod] = rawMod.split('.')
        ctx.mods.set(label, new Set(mod))
      }

      const cleanup = plugin.apply(ctx)
      if (cleanup) {
        let cleanups = removals.get(el)
        if (cleanups) {
          cleanups.get(rawKey)?.()
        } else {
          cleanups = new Map()
          removals.set(el, cleanups)
        }
        cleanups.set(rawKey, cleanup)
      }
    }
  }
}

type GenRxOptions = {
  returnsValue?: boolean
  argNames?: string[]
}

type GenRxFn = <T>(el: HTMLOrSVG, ...args: any[]) => T

const genRx = (
  value: string,
  { returnsValue = false, argNames = [] }: GenRxOptions = {},
): GenRxFn => {
  let expr = ''
  if (returnsValue) {
    // This regex allows Datastar expressions to support nested
    // regex and strings that contain ; without breaking.
    //
    // Each of these regex defines a block type we want to match
    // (importantly we ignore the content within these blocks):
    //
    // regex            \/(\\\/|[^\/])*\/
    // double quotes      "(\\"|[^\"])*"
    // single quotes      '(\\'|[^'])*'
    // ticks              `(\\`|[^`])*`
    // iife               \(\s*((function)\s*\(\s*\)|(\(\s*\))\s*=>)\s*(?:\{[\s\S]*?\}|[^;)\{]*)\s*\)\s*\(\s*\)
    //
    // The iife support is (intentionally) limited. It only supports
    // function and arrow syntax with no arguments, and no nested IIFEs.
    //
    // We also want to match the non delimiter part of statements
    // note we only support ; statement delimiters:
    //
    // [^;]
    //
    const statementRe =
      /(\/(\\\/|[^/])*\/|"(\\"|[^"])*"|'(\\'|[^'])*'|`(\\`|[^`])*`|\(\s*((function)\s*\(\s*\)|(\(\s*\))\s*=>)\s*(?:\{[\s\S]*?\}|[^;){]*)\s*\)\s*\(\s*\)|[^;])+/gm
    const statements = value.trim().match(statementRe)
    if (statements) {
      const lastIdx = statements.length - 1
      const last = statements[lastIdx].trim()
      if (!last.startsWith('return')) {
        statements[lastIdx] = `return (${last});`
      }
      expr = statements.join(';\n')
    }
  } else {
    expr = value.trim()
  }

  // Ignore any escaped values
  const escaped = new Map<string, string>()
  const escapeRe = RegExp(`(?:${DSP})(.*?)(?:${DSS})`, 'gm')
  let counter = 0
  for (const match of expr.matchAll(escapeRe)) {
    const k = match[1]
    const v = `__escaped${counter++}`
    escaped.set(v, k)
    expr = expr.replace(DSP + k + DSS, v)
  }

  // Replace signal references with bracket notation
  // Examples:
  //   $count          -> $['count']
  //   $count--        -> $['count']--
  //   $foo.bar        -> $['foo']['bar']
  //   $foo-bar        -> $['foo-bar']
  //   $foo.bar-baz    -> $['foo']['bar-baz']
  //   $foo-$bar       -> $['foo']-$['bar']
  //   $arr[$index]    -> $['arr'][$['index']]
  //   $['foo']        -> $['foo']
  //   $foo[obj.bar]   -> $['foo'][obj.bar]
  //   $foo['bar.baz'] -> $['foo']['bar.baz']
  //   $123            -> $['123']
  //   $foo.0.name     -> $['foo']['0']['name']

  expr = expr
    // $['x'] -> $x (normalize existing bracket notation)
    .replace(/\$\['([a-zA-Z_$\d][\w$]*)'\]/g, '$$$1')
    // $x -> $['x'] (including dots and hyphens)
    .replace(/\$([a-zA-Z_\d]\w*(?:[.-]\w+)*)/g, (_, signalName) =>
      signalName
        .split('.')
        .reduce((acc: string, part: string) => `${acc}['${part}']`, '$'),
    )
    // [$x] -> [$['x']] ($ inside brackets)
    .replace(
      /\[(\$[a-zA-Z_\d]\w*)\]/g,
      (_, varName) => `[$['${varName.slice(1)}']]`,
    )

  expr = expr.replaceAll(/@(\w+)\(/g, '__action("$1",evt,')

  // Replace any escaped values
  for (const [k, v] of escaped) {
    expr = expr.replace(k, v)
  }

  try {
    expr = `with(this){${expr}}`
    const fn = Function('el', '$', '__action', 'evt', ...argNames, expr)
    return (el: HTMLOrSVG, ...args: any[]) => {
      const action = (name: string, evt: Event | undefined, ...args: any[]) => {
        const err = error.bind(0, {
          plugin: { type: 'action', name },
          element: { id: el.id, tag: el.tagName },
          expression: {
            fnContent: expr,
            value,
          },
        })
        const fn = actions[name]
        if (fn) {
          return fn(
            {
              el,
              evt,
              error: err,
            },
            ...args,
          )
        }
        throw err('UndefinedAction')
      }
      try {
        const store = getStoreFor(el)
        const host = getHostFor(el)
        return fn.call(host, el, store, action, undefined, ...args)
      } catch (e: any) {
        console.error(e)
        throw error(
          {
            element: { id: el.id, tag: el.tagName },
            expression: {
              fnContent: expr,
              value,
            },
            error: e.message,
          },
          'ExecuteExpression',
        )
      }
    }
  } catch (e: any) {
    console.error(e)
    throw error(
      {
        expression: {
          fnContent: expr,
          value,
        },
        error: e.message,
      },
      'GenerateExpression',
    )
  }
}
