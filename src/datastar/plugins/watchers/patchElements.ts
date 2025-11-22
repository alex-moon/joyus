// Icon: material-symbols:cloud-download
// Slug: Patches elements into the DOM.
// Description: Patches elements into the DOM.

import { watcher } from '@engine'
import type {DatastarElementPatchEvent, WatcherContext} from '@engine/types'
import { supportsViewTransitions } from '@utils/view-transitions'
import {getHostFor} from "@engine/signals";
import {DATASTAR_ELEMENT_PATCH_EVENT} from "@engine/consts";

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
    if (!child.id) {
      console.warn(error('PatchElementsNoTargetsFound'), {
        element: { id: child.id },
      })
      continue
    }

    document.dispatchEvent(
        new CustomEvent<DatastarElementPatchEvent>(DATASTAR_ELEMENT_PATCH_EVENT, {
            detail: {
              id: child.id,
              element: child,
            },
            // @todo do we need these??
            bubbles: true,
            composed: true,
        }),
    )
  }
}
