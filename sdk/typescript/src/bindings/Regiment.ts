import {types} from "ref-napi";
import * as ref from "ref-napi";
import {Model, ModelDto, dehydrate as dehydrateModel, hydrate as hydrateModel} from "./Model";

const StructType = require("ref-struct-di")(ref);

export type Regiment = {
    model: Model
    nb_rows: number
    nb_cols: number
    nb_models: number
    regiment_health_point: number
    points: number
}

export const RegimentDto = StructType({
    model: ModelDto,
    nb_rows: types.uint64,
    nb_cols: types.uint64,
    nb_models: types.uint64,
    regiment_health_point: types.uint64,
    points: types.uint64
});

export const hydrate = (regiment: typeof RegimentDto): Regiment => ({
    model: hydrateModel(regiment.model),
    nb_rows: regiment.nb_rows,
    nb_cols: regiment.nb_cols,
    nb_models: regiment.nb_models,
    regiment_health_point: regiment.regiment_health_point,
    points: regiment.points
});

export const dehydrate = (regiment: Regiment): typeof RegimentDto => ({
    model: dehydrateModel(regiment.model),
    nb_rows: regiment.nb_rows,
    nb_cols: regiment.nb_cols,
    nb_models: regiment.nb_models,
    regiment_health_point: regiment.regiment_health_point,
    points: regiment.points
});
