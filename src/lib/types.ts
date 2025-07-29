export enum Family {
  Scripting = "Scripting",
  Web = "Web",
  Mobile = "Mobile",
  Gaming = "Gaming",
  Database = "Database",
  Systems = "Systems",
  Neural = "Neural",
  Mythical = "Mythical",
}

export interface DexMon {
  id: number;
  name: string;
  title: string;
  family: Family;
  appearance: string;
  description: string;
  rarity: number;
  collect_cmds: string[];
  exp_cmds: string[];
}

export interface PartyMon {
  dex_id: number;
  level: number;
  experience_range: [number, number];
}
