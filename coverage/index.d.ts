export interface TerrainGrid {
  width: number;
  height: number;
  minLat: number;
  minLon: number;
  maxLat: number;
  maxLon: number;
  elevation: number[];
  surface: number[];
  zone: number[];
  sourceId?: string;
  resolutionM?: number;
}

export interface CoverageParams {
  centerLat: number;
  centerLon: number;
  radiusKm: number;
  resolutionM: number;
  frequencyGhz: number;
  txHeightM: number;
  rxHeightM?: number;
  timePercent?: number;
  dn: number;
  n0: number;
  /** 1 = horizontal (default), 2 = vertical */
  polarization?: number;
}

export interface CoverageResult {
  width: number;
  height: number;
  minLat: number;
  minLon: number;
  maxLat: number;
  maxLon: number;
  resolutionM: number;
  data: number[];
  sourceIds: string[];
}

export declare function calculateCoverage(params: CoverageParams): CoverageResult;

export declare function calculateCoverageWithTerrain(
  params: CoverageParams,
  terrain: TerrainGrid,
): CoverageResult;
