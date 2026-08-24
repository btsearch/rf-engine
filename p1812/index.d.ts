export interface PathProfile {
  distanceKm: number[];
  terrainM: number[];
  surfaceM: number[];
  zone: number[];
}

export interface P1812Params {
  frequencyGhz: number;
  txHeightM: number;
  rxHeightM: number;
  timePercent: number;
  locationPercent?: number;
  sigmaL?: number;
  txLat: number;
  txLon: number;
  rxLat: number;
  rxLon: number;
  dn: number;
  n0: number;
  /** 1 = horizontal (default), 2 = vertical */
  polarization?: number;
}

export interface P1812Result {
  basicTransmissionLossDb: number;
  fieldStrengthDbuvm: number;
}

export interface P1812DetailedResult {
  basicTransmissionLossDb: number;
  fieldStrengthDbuvm: number;
  pathType: "los" | "transhorizon";
  freeSpaceLossDb: number;
  diffractionLossDb: number;
  troposcatterLossDb: number;
  anomalousLossDb: number;
  txHorizonDistanceKm: number;
  rxHorizonDistanceKm: number;
  effectiveEarthRadiusKm: number;
  beta0: number;
  seaFraction: number;
  bullingtonDistanceKm: number | null;
}

export declare function calculateP1812(profile: PathProfile, params: P1812Params): P1812Result;

export declare function calculateP1812Detailed(
  profile: PathProfile,
  params: P1812Params,
): P1812DetailedResult;
