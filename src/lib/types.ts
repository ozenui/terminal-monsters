export enum Family {
  Scripting = "Scripting",
  Web = "Web",
  Mobile = "Mobile",
  Gaming = "Gaming",
  Database = "Database",
  Systems = "Systems",
  Neural = "Neural",
  Ancient = "Ancient",
}

export interface DexMon {
  id: number;
  name: string;
  family: Family;
  rarity: number;
  collect_cmds: string[];
  exp_commands: string[];
}

export interface PartyMon {
  dex_id: number;
  level: number;
  experience_range: [number, number];
}
