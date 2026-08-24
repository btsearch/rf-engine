# @btsearch/rf-engine

RF propagation engine with pluggable terrain sources, ITU-R P.1812-8 prediction, coverage mapping, and link budget calculations. Native Node.js module built with Rust and [napi-rs](https://napi.rs).

## Install

```
npm install @btsearch/rf-engine
```

Platform-specific native binaries are installed automatically for Linux (x64, arm64), macOS (x64, arm64), and Windows (x64).

## Subpath exports

```ts
import { calculateP1812, calculateP1812Detailed } from '@btsearch/rf-engine/p1812'
import { calculateLinkBudget } from '@btsearch/rf-engine/link-budget'
import { calculateCoverage, calculateCoverageWithTerrain } from '@btsearch/rf-engine/coverage'
```

## P.1812 propagation prediction

```ts
import { calculateP1812Detailed } from '@btsearch/rf-engine/p1812'

const result = calculateP1812Detailed(
  {
    distanceKm: [0, 0.1, 0.2, /* ... */],
    terrainM: [120, 118, 115, /* ... */],
    surfaceM: [125, 123, 120, /* ... */],
    zone: [4, 4, 4, /* ... */], // 1=sea, 3=coastal, 4=inland
  },
  {
    frequencyGhz: 0.9,
    txHeightM: 30,
    rxHeightM: 1.5,
    timePercent: 50,
    txLat: 50.0, txLon: 19.0,
    rxLat: 50.08, rxLon: 19.0,
    dn: 45, n0: 325,
  },
)

console.log(result.basicTransmissionLossDb)  // 127.3
console.log(result.pathType)                  // "los"
console.log(result.diffractionLossDb)         // 15.9
```

## Coverage with terrain data

Pass a terrain elevation grid and get a parallel-computed coverage raster:

```ts
import { calculateCoverageWithTerrain } from '@btsearch/rf-engine/coverage'

const result = calculateCoverageWithTerrain(
  {
    centerLat: 50.26, centerLon: 19.02,
    radiusKm: 10, resolutionM: 30,
    frequencyGhz: 0.8, txHeightM: 40,
    dn: 45, n0: 325,
  },
  {
    width: 667, height: 667,
    minLat: 50.17, minLon: 18.88,
    maxLat: 50.35, maxLon: 19.16,
    elevation: new Float64Array(/* DTM */),
    surface: new Float64Array(/* DSM */),
    zone: new Uint8Array(/* zones */),
    sourceId: 'gugik-lidar',
  },
)

console.log(`${result.width}x${result.height} pixels`)
console.log(`sources: ${result.sourceIds}`)
```

## Link budget

```ts
import { calculateLinkBudget } from '@btsearch/rf-engine/link-budget'

const lb = calculateLinkBudget({
  txPowerDbm: 43,
  txGainDbi: 17,
  rxSensitivityDbm: -100,
  pathLossDb: 130,
})

console.log(`Received: ${lb.receivedPowerDbm} dBm, Margin: ${lb.marginDb} dB`)
```

## How it works

The propagation math runs in Rust using the [p1812](https://crates.io/crates/p1812) crate, exposed to Node.js through napi-rs with zero-copy typed array support. Coverage calculations use Rayon to parallelize across all CPU cores.

## License

MIT
