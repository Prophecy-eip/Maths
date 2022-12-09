import {ForeignFunction, Library} from "ffi-napi";

import * as ref from "ref-napi";
import {RegimentDto, Regiment, dehydrate as dehydrateRegiment} from "./bindings/Regiment";
import {Prediction, PredictionDto, hydrate as hydratePrediction} from "./bindings/Prediction";

const StructType = require("ref-struct-di")(ref);

type ReturnType = {
    best_prediction: Prediction;
    worst_prediction: Prediction;
    mean_prediction: Prediction;
}

const ReturnTypeDto = StructType({
    best_prediction: PredictionDto,
    worst_prediction: PredictionDto,
    mean_prediction: PredictionDto
});

export class Sdk {
    private readonly _library: {
        compute_fight: ForeignFunction<typeof ReturnTypeDto, [typeof RegimentDto, typeof RegimentDto]>;
    };

    constructor(libraryPath: string) {
        this._library = Library(libraryPath, {
            compute_fight: [ReturnTypeDto, [RegimentDto, RegimentDto]],
        });
    }

    public computeFight(attackingRegiment: Regiment, defendingRegiment: Regiment): void {
        this._library.compute_fight(
            dehydrateRegiment(attackingRegiment),
            dehydrateRegiment(defendingRegiment)
        );
    }

    private static hydrateReturnType(returnType: typeof ReturnTypeDto): ReturnType {
        return {
            best_prediction: hydratePrediction(returnType.best_prediction),
            worst_prediction: hydratePrediction(returnType.worst_prediction),
            mean_prediction: hydratePrediction(returnType.mean_prediction)
        }
    }
}
