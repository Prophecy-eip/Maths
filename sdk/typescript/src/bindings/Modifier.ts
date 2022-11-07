import {types} from "ref-napi";
import * as ref from "ref-napi";
import {Stats, StatsDto, dehydrate as dehydrateStats} from "./Stats";
import {Requirement, dehydrate as dehydrateRequirement, RequirementDto, hydrate as hydrateRequirement} from "./Requirement";

const StructType = require("ref-struct-di")(ref);
const ArrayType = require("ref-array-di")(ref);

const RequirementArray = ArrayType(RequirementDto);

export type Modifier = {
    stat: Stats,
    bonus: boolean,
    nb_dice: number,
    requirements: Requirement[],
};

export const ModifierDto = StructType({
    stat: StatsDto,
    bonus: types.bool,
    nb_dice: types.uint64,
    requirements_size: types.uint64,
    requirements: RequirementArray,
});

export const hydrate = (modifier: typeof ModifierDto): Modifier => ({
    stat: modifier.stat,
    bonus: modifier.bonus,
    nb_dice: modifier.nb_dice,
    requirements: modifier.requirements.slice(0, modifier.requirements_size).map(hydrateRequirement),
});

export const dehydrate = (modifier: Modifier): typeof ModifierDto => ({
    stat: dehydrateStats(modifier.stat),
    bonus: modifier.bonus,
    nb_dice: modifier.nb_dice,
    requirements_size: modifier.requirements.length,
    requirements: new RequirementArray(modifier.requirements.map(dehydrateRequirement)),
});
