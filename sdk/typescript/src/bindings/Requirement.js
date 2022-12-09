"use strict";
exports.__esModule = true;
exports.dehydrate = exports.hydrate = exports.RequirementDto = exports.Requirement = void 0;
var ref_napi_1 = require("ref-napi");
var Requirement;
(function (Requirement) {
    Requirement[Requirement["WeaknessToFire"] = 0] = "WeaknessToFire";
    Requirement[Requirement["WeaknessToMagic"] = 1] = "WeaknessToMagic";
    Requirement[Requirement["WeaknessToIce"] = 2] = "WeaknessToIce";
})(Requirement = exports.Requirement || (exports.Requirement = {}));
exports.RequirementDto = ref_napi_1.types.uint16;
var hydrate = function (requirement) {
    switch (requirement) {
        case Requirement.WeaknessToFire:
            return Requirement.WeaknessToFire;
        case Requirement.WeaknessToMagic:
            return Requirement.WeaknessToMagic;
        case Requirement.WeaknessToIce:
            return Requirement.WeaknessToIce;
    }
};
exports.hydrate = hydrate;
var dehydrate = function (requirement) {
    console.log("dehydrate requirement", requirement);
    return requirement;
};
exports.dehydrate = dehydrate;
