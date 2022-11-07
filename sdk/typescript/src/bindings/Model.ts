import {types} from "ref-napi";
import * as ref from "ref-napi";
import {Stats, StatsDto, dehydrate as dehydrateStats} from "./Stats";
import {Modifier, ModifierDto, dehydrate as dehydrateModifier, hydrate as hydrateModifier} from "./Modifier";

const StructType = require("ref-struct-di")(ref);
const ArrayType = require("ref-array-di")(ref);

const ModifierArray = ArrayType(ModifierDto);

export type Model = {
    stats: Stats,
    modifiers: Modifier[],
}

export const ModelDto = StructType({
    stats: StatsDto,
    size: types.uint64,
    modifiers: ModifierArray,
});

export const hydrate = (model: typeof ModelDto): Model => ({
    stats: model.stats,
    modifiers: model.modifiers.slice(0, model.size).map(hydrateModifier),
});

export const dehydrate = (model: Model): typeof ModelDto => ({
    stats: dehydrateStats(model.stats),
    size: model.modifiers.length,
    modifiers: new ModifierArray(model.modifiers.map(dehydrateModifier)),
});
