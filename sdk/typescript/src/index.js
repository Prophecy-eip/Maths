"use strict";
exports.__esModule = true;
var sdk_1 = require("./sdk");
var test = new sdk_1.Sdk("/Users/pepiloto/Epitech/EIP/Maths/target/debug/libmaths.dylib");
var regiment = {
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
            offensive: 4,
            resilience: 4,
            strength: 5,
            health_point: 1,
            armour_penetration: 1
        },
        modifiers: []
    },
    nb_rows: 4,
    nb_cols: 5,
    nb_models: 20,
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
console.log(test.computeFight(regiment, regiment));
