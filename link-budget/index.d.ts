export interface LinkBudgetParams {
  txPowerDbm: number;
  txGainDbi?: number;
  txFeederLossDb?: number;
  rxGainDbi?: number;
  rxFeederLossDb?: number;
  rxSensitivityDbm: number;
  pathLossDb: number;
}

export interface LinkBudgetResult {
  eirpDbm: number;
  pathLossDb: number;
  receivedPowerDbm: number;
  marginDb: number;
}

export declare function calculateLinkBudget(params: LinkBudgetParams): LinkBudgetResult;
