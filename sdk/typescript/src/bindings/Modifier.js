"use strict";
exports.__esModule = true;
exports.dehydrate = exports.hydrate = exports.ModifierDto = void 0;
var ref_napi_1 = require("ref-napi");
var ref = require("ref-napi");
var Stats_1 = require("./Stats");
var Requirement_1 = require("./Requirement");
var StructType = require("ref-struct-di")(ref);
var ArrayType = require("ref-array-di")(ref);
var RequirementArray = ArrayType(Requirement_1.RequirementDto);
exports.ModifierDto = StructType({
    stat: Stats_1.StatsDto,
    bonus: ref_napi_1.types.bool,
    nb_dice: ref_napi_1.types.uint64,
    requirements_size: ref_napi_1.types.uint64,
    requirements: RequirementArray
});
var hydrate = function (modifier) { return ({
    stat: modifier.stat,
    bonus: modifier.bonus,
    nb_dice: modifier.nb_dice,
    requirements: modifier.requirements.slice(0, modifier.requirements_size).map(Requirement_1.hydrate)
}); };
exports.hydrate = hydrate;
var dehydrate = function (modifier) { return ({
    stat: (0, Stats_1.dehydrate)(modifier.stat),
    bonus: modifier.bonus,
    nb_dice: modifier.nb_dice,
    requirements_size: modifier.requirements.length,
    requirements: new RequirementArray(modifier.requirements.map(Requirement_1.dehydrate))
}); };
exports.dehydrate = dehydrate;
