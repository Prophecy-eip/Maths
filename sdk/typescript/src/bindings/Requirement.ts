import {types} from "ref-napi";

export enum Requirement {
    WeaknessToFire = 0,
    WeaknessToMagic = 1,
    WeaknessToIce = 2
}

export const RequirementDto = types.uint16

export const hydrate = (requirement: typeof RequirementDto): Requirement => {
    switch (requirement as unknown as Requirement) {
        case Requirement.WeaknessToFire:
            return Requirement.WeaknessToFire;
        case Requirement.WeaknessToMagic:
            return Requirement.WeaknessToMagic;
        case Requirement.WeaknessToIce:
            return Requirement.WeaknessToIce;
    }
}

export const dehydrate = (requirement: Requirement): typeof RequirementDto => {
    console.log("dehydrate requirement", requirement)
    return requirement as any
}
