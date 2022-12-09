"use strict";
exports.__esModule = true;
exports.Sdk = void 0;
var ffi_napi_1 = require("ffi-napi");
var ref = require("ref-napi");
var Regiment_1 = require("./bindings/Regiment");
var Prediction_1 = require("./bindings/Prediction");
var StructType = require("ref-struct-di")(ref);
var ReturnTypeDto = StructType({
    best_prediction: Prediction_1.PredictionDto,
    worst_prediction: Prediction_1.PredictionDto,
    mean_prediction: Prediction_1.PredictionDto
});
var Sdk = /** @class */ (function () {
    function Sdk(libraryPath) {
        this._library = (0, ffi_napi_1.Library)(libraryPath, {
            compute_fight: [ReturnTypeDto, [Regiment_1.RegimentDto, Regiment_1.RegimentDto]]
        });
    }
    Sdk.prototype.computeFight = function (attackingRegiment, defendingRegiment) {
        this._library.compute_fight((0, Regiment_1.dehydrate)(attackingRegiment), (0, Regiment_1.dehydrate)(defendingRegiment));
    };
    Sdk.hydrateReturnType = function (returnType) {
        return {
            best_prediction: (0, Prediction_1.hydrate)(returnType.best_prediction),
            worst_prediction: (0, Prediction_1.hydrate)(returnType.worst_prediction),
            mean_prediction: (0, Prediction_1.hydrate)(returnType.mean_prediction)
        };
    };
    return Sdk;
}());
exports.Sdk = Sdk;
