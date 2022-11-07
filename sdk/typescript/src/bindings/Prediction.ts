import {types} from "ref-napi";
import * as ref from "ref-napi";
import {Regiment, RegimentDto, hydrate as hydrateRegiment} from "./Regiment";

const StructType = require("ref-struct-di")(ref);

export type Prediction = {
    //attacking_regiment: Regiment,
    //defending_regiment: Regiment,
    occurence_probability: number
};

export const PredictionDto = StructType({
    //attacking_regiment: RegimentDto,
    //defending_regiment: RegimentDto,
    occurence_probability: types.double
});

export const hydrate = (prediction: typeof PredictionDto): Prediction => ({
    //attacking_regiment: hydrateRegiment(prediction.attacking_regiment),
    //defending_regiment: hydrateRegiment(prediction.defending_regiment),
    occurence_probability: prediction.occurence_probability
});
