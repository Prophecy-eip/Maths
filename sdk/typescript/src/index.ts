import {Sdk} from "./sdk";
import {Regiment} from "./bindings/Regiment";

const test = new Sdk("C:\\Users\\Aurélien BOCH\\Documents\\Code\\Prophecy\\Maths\\target\\release\\maths.dll");

const regiment: Regiment = {
    model: {
        stats: {
            advance: 6,
            agility: 4,
            armour: 4,
            aegis: 4,
            attack: 4,
            defense: 4,
            march: 6,
            discipline: 4,
            offensive: 4,
            resilience: 4,
            strength: 4,
            health_point: 4,
            armour_penetration: 4,
        },
        modifiers: [],
    },
    nb_rows: 1,
    nb_cols: 10,
    nb_models: 1,
    regiment_health_point: 1,
    points: 1
};

/*const regiment: Regiment = {
    model: {
        stats: {
            advance: 4,
            agility: 4,
            armour: 0,
            aegis: 0,
            attack: 2,
            defense: 5,
            march: 8,
            discipline: 8,
            offensive: 5,
            resilience: 4,
            strength: 5,
            health_point: 1,
            armour_penetration: 1,
        },
        modifiers: [],
    },
    nb_rows: 1,
    nb_cols: 1,
    nb_models: 20,
    regiment_health_point: 0,
    points: 0
};*/

console.log(test.computeFight(
    regiment,
    regiment,
));
