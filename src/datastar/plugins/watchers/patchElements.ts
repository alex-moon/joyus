// Icon: material-symbols:cloud-download
// Slug: Patches elements into the DOM.
// Description: Patches elements into the DOM.

import { watcher } from '@engine'
import type { WatcherContext } from '@engine/types'
import { supportsViewTransitions } from '@utils/view-transitions'
import {getHostFor} from "@engine/signals";

type PatchElementsArgs = {
  elements: string
  useViewTransition: boolean
}

watcher({
  name: 'datastar-patch-elements',
  apply(
    ctx,
    { elements = '', useViewTransition },
  ) {
    const args: PatchElementsArgs = {
      elements,
      useViewTransition: useViewTransition?.trim() === 'true',
    }

    if (supportsViewTransitions && useViewTransition) {
      document.startViewTransition(() => onPatchElements(ctx, args))
    } else {
      onPatchElements(ctx, args)
    }
  },
})

const onPatchElements = (
  { el, error }: WatcherContext,
  { elements }: PatchElementsArgs,
) => {
  const newDocument = new DOMParser().parseFromString(
    `<body><template>${elements}</template></body>`,
    'text/html',
  )

  const newContent = newDocument.querySelector('template')!.content

  for (const child of newContent.children) {
    const target = getHostFor(el)!
    if (!target) {
      console.warn(error('PatchElementsNoTargetsFound'), {
        element: { id: child.id },
      })
      continue
    }

    applyToTargets(child, [target])
  }
}

const scripts = new WeakSet<HTMLScriptElement>()
for (const script of document.querySelectorAll('script')) {
  scripts.add(script)
}

const execute = (target: Element): void => {
  const elScripts =
    target instanceof HTMLScriptElement
      ? [target]
      : target.querySelectorAll('script')
  for (const old of elScripts) {
    if (!scripts.has(old)) {
      const script = document.createElement('script')
      for (const { name, value } of old.attributes) {
        script.setAttribute(name, value)
      }
      script.text = old.text
      old.replaceWith(script)
      scripts.add(script)
    }
  }
}

const applyToTargets = (
  element: DocumentFragment | Element,
  targets: Iterable<Element>,
) => {
  for (const target of targets) {
    const cloned = element.cloneNode(true) as Element
    execute(cloned)
    target.replaceWith(cloned)
  }
}
