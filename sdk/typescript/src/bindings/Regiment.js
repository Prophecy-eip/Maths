"use strict";
exports.__esModule = true;
exports.dehydrate = exports.hydrate = exports.RegimentDto = void 0;
var ref_napi_1 = require("ref-napi");
var ref = require("ref-napi");
var Model_1 = require("./Model");
var StructType = require("ref-struct-di")(ref);
exports.RegimentDto = StructType({
    model: Model_1.ModelDto,
    nb_rows: ref_napi_1.types.uint64,
    nb_cols: ref_napi_1.types.uint64,
    nb_models: ref_napi_1.types.uint64,
    regiment_health_point: ref_napi_1.types.uint64,
    points: ref_napi_1.types.uint64
});
var hydrate = function (regiment) { return ({
    model: (0, Model_1.hydrate)(regiment.model),
    nb_rows: regiment.nb_rows,
    nb_cols: regiment.nb_cols,
    nb_models: regiment.nb_models,
    regiment_health_point: regiment.regiment_health_point,
    points: regiment.points
}); };
exports.hydrate = hydrate;
var dehydrate = function (regiment) { return ({
    model: (0, Model_1.dehydrate)(regiment.model),
    nb_rows: regiment.nb_rows,
    nb_cols: regiment.nb_cols,
    nb_models: regiment.nb_models,
    regiment_health_point: regiment.regiment_health_point,
    points: regiment.points
}); };
exports.dehydrate = dehydrate;
