"use strict";
exports.__esModule = true;
exports.dehydrate = exports.StatsDto = void 0;
var ref_napi_1 = require("ref-napi");
var ref = require("ref-napi");
var StructType = require("ref-struct-di")(ref);
exports.StatsDto = StructType({
    advance: ref_napi_1.types.uint64,
    march: ref_napi_1.types.uint64,
    discipline: ref_napi_1.types.uint64,
    health_point: ref_napi_1.types.uint64,
    defense: ref_napi_1.types.uint64,
    resilience: ref_napi_1.types.uint64,
    armour: ref_napi_1.types.uint64,
    aegis: ref_napi_1.types.uint64,
    attack: ref_napi_1.types.uint64,
    offensive: ref_napi_1.types.uint64,
    strength: ref_napi_1.types.uint64,
    armour_penetration: ref_napi_1.types.uint64,
    agility: ref_napi_1.types.uint64
});
var dehydrate = function (stats) { return ({
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
}); };
exports.dehydrate = dehydrate;
