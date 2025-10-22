// Icon: streamline:wifi-signal-full-remix
// Slug: Patches signals into the existing signals.
// Description: Patches (adds, updates or removes) one or more signals into the existing signals.

import { attribute } from '@engine'
import {getStoreFor, mergePatch, mergePaths} from '@engine/signals'
import { modifyCasing } from '@utils/text'

attribute({
  name: 'signals',
  returnsValue: true,
  apply({ el, key, mods, rx }) {
    const store = getStoreFor(el)
    const ifMissing = mods.has('ifmissing')

    if (key) {
      key = modifyCasing(key, mods)
      mergePaths([[key, rx?.()]], store, {ifMissing})
    } else {
      const patch = Object.assign({}, rx?.() as Record<string, any>)
      mergePatch(patch, store, {ifMissing})
    }
  },
})
