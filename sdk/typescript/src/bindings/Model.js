"use strict";
exports.__esModule = true;
exports.dehydrate = exports.hydrate = exports.ModelDto = void 0;
var ref_napi_1 = require("ref-napi");
var ref = require("ref-napi");
var Stats_1 = require("./Stats");
var Modifier_1 = require("./Modifier");
var StructType = require("ref-struct-di")(ref);
var ArrayType = require("ref-array-di")(ref);
var ModifierArray = ArrayType(Modifier_1.ModifierDto);
exports.ModelDto = StructType({
    stats: Stats_1.StatsDto,
    size: ref_napi_1.types.uint64,
    modifiers: ModifierArray
});
var hydrate = function (model) { return ({
    stats: model.stats,
    modifiers: model.modifiers.slice(0, model.size).map(Modifier_1.hydrate)
}); };
exports.hydrate = hydrate;
var dehydrate = function (model) { return ({
    stats: (0, Stats_1.dehydrate)(model.stats),
    size: model.modifiers.length,
    modifiers: new ModifierArray(model.modifiers.map(Modifier_1.dehydrate))
}); };
exports.dehydrate = dehydrate;
