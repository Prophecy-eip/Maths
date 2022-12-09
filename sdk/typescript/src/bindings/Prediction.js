"use strict";
exports.__esModule = true;
exports.hydrate = exports.PredictionDto = void 0;
var ref_napi_1 = require("ref-napi");
var ref = require("ref-napi");
var StructType = require("ref-struct-di")(ref);
exports.PredictionDto = StructType({
    //attacking_regiment: RegimentDto,
    //defending_regiment: RegimentDto,
    occurence_probability: ref_napi_1.types.double
});
var hydrate = function (prediction) { return ({
    //attacking_regiment: hydrateRegiment(prediction.attacking_regiment),
    //defending_regiment: hydrateRegiment(prediction.defending_regiment),
    occurence_probability: prediction.occurence_probability
}); };
exports.hydrate = hydrate;
