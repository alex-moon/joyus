// Icon: si:json-fill
// Slug: Outputs a JSON stringified version of signals.
// Description: Sets the text content of an element to a reactive JSON stringified version of signals.

import { attribute } from '@engine'
import {effect, filtered, getStoreFor} from '@engine/signals'
import type { SignalFilterOptions } from '@engine/types'
import { jsStrToObject } from '@utils/text'

attribute({
  name: 'json-signals',
  requirement: {
    key: 'denied',
  },
  apply({ el, value, mods }) {
    const store = getStoreFor(el)
    const spaces = mods.has('terse') ? 0 : 2
    let filters: SignalFilterOptions = {}
    if (value) {
      filters = jsStrToObject(value)
    }

    const callback = () => {
      observer.disconnect()
      el.textContent = JSON.stringify(filtered(filters, store), null, spaces)
      observer.observe(el, {
        childList: true,
        characterData: true,
        subtree: true,
      })
    }
    const observer = new MutationObserver(callback)
    const cleanup = effect(callback)

    return () => {
      observer.disconnect()
      cleanup()
    }
  },
})
