import {types} from "ref-napi";
import * as ref from "ref-napi";

const StructType = require("ref-struct-di")(ref);

export type Stats = {
    advance: number,
    march: number,
    discipline: number,
    health_point: number,
    defense: number,
    resilience: number,
    armour: number,
    aegis: number,
    attack: number,
    offensive: number,
    strength: number,
    armour_penetration: number,
    agility: number
}

export const StatsDto = StructType({
    advance: types.uint64,
    march: types.uint64,
    discipline: types.uint64,
    health_point: types.uint64,
    defense: types.uint64,
    resilience: types.uint64,
    armour: types.uint64,
    aegis: types.uint64,
    attack: types.uint64,
    offensive: types.uint64,
    strength: types.uint64,
    armour_penetration: types.uint64,
    agility: types.uint64
});

export const dehydrate = (stats: Stats): typeof StatsDto => ({
    advance: stats.advance,
    march: stats.march,
    discipline: stats.discipline,
    health_point: stats.health_point,
    defense: stats.defense,
    resilience: stats.resilience,
    armour: stats.armour,
    aegis: stats.aegis,
    attack: stats.attack,
    offensive: stats.offensive,
    strength: stats.strength,
    armour_penetration: stats.armour_penetration,
    agility: stats.agility
});
