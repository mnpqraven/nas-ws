type Banner = {
    bannerName: string
    banner: number
    guaranteed: number
    guaranteedPity?: number
    minConst: number
    maxConst: number
    maxPity: number
    constFormat: string
    constName: string
    rate: (pity: number) => number
}

type Sim = ReducedSim & {
    pity: number
    guaranteed: boolean
    guaranteedPity: number
}
type ReducedSim = {
    const: number
    rate: number
}
function pityRate(baseRate: number, pityStart: number): (pity: number) => number {
    return (pity) => pity < pityStart ? baseRate : baseRate + baseRate * 10 * (pity - pityStart + 1)
}

const gachas: Record<string, Banner> = {
    char: {
        bannerName: "5* Banner character",
        banner: 0.5,
        guaranteed: 1,
        minConst: -1,
        maxConst: 6,
        constFormat: "C",
        constName: "Constellations",
        maxPity: 90,
        rate: pityRate(0.6, 74)
    },
    "4*char": {
        bannerName: "Specific 4* banner character",
        banner: 0.5,
        guaranteed: 1 / 3,
        minConst: -1,
        maxConst: 6,
        constFormat: "C",
        constName: "Constellations",
        maxPity: 10,
        rate: pityRate(5.1, 9)
    },
    weapon: {
        bannerName: "Specific 5* banner weapon",
        banner: 0.75,
        guaranteed: 1 / 2,
        guaranteedPity: 3,
        minConst: 0,
        maxConst: 5,
        constFormat: "R",
        constName: "Refinements",
        maxPity: 80,
        rate: pityRate(0.7, 63)
    }
}

function calcSimsRegular(current: number, pity: number, pulls: number, guaranteed: boolean, guaranteedPity: number, banner: Banner): ReducedSim[][] {
    return calcSimsInt({
        pity,
        guaranteed,
        guaranteedPity,
        const: current,
        rate: 1
    }, pulls, banner)
}

function calcSimsInt(starterSim: Sim, pulls: number, banner: Banner): ReducedSim[][] {
    console.time("calc")
    const sims: Sim[][] = calcSimsExact([starterSim], pulls, banner, 0)
    console.timeEnd("calc")


    return sims.map(sim => {
        // Reducing to simple sims with less information
        const reducedSims: ReducedSim[] = []
        sim.forEach((sim: Sim) => {
            if (sim.rate == 0) return

            const other = reducedSims[sim.const + 1]

            if (other)
                other.rate += sim.rate
            else
                reducedSims[sim.const + 1] = {
                    const: sim.const,
                    rate: sim.rate
                }
        })
        return reducedSims
    })
}

function calcSimsExact(sims: Sim[], pulls: number, banner: Banner, prune = 1e-8) {
    const allSims: Sim[][] = [sims]
    for (let i = 0; i < pulls; i++) {
        const newSims: Record<number, Sim> = {}

        const addOrMerge = (sim: Sim) => {
            if (sim.rate <= 0) return

            const v = sim.pity + (banner.maxPity + 1) * ((sim.const + 1) + ((banner.maxConst + 2) * (+sim.guaranteed + (2 * sim.guaranteedPity))))
            const other = newSims[v]

            if (other) {
                // if (other.const != sim.const) console.error("const", v, sim, other)
                // if (other.guaranteed != sim.guaranteed) console.error("guaranteed", v, sim, other)
                // if (other.guaranteedPity != sim.guaranteedPity) console.error("guaranteedPity", v, sim, other)
                // if (other.pity != sim.pity) console.error("pity", v, sim, other)

                other.rate += sim.rate
                return
            }

            newSims[v] = sim
        }

        for (const sim of sims) {

            if (!sim) continue
            if (sim.rate <= prune) continue // Pruning
            if (sim.const >= banner.maxConst) { // Limited to C6
                addOrMerge({ ...sim })
                continue
            }
            const currentPity = sim.pity + 1
            let rate = banner.rate(currentPity) / 100
            if (rate > 1) rate = 1
            else if (rate < 0) rate = 0
            const bannerRate = (
                sim.guaranteed ||
                (banner.guaranteedPity && sim.guaranteedPity >= banner.guaranteedPity - 1)
            ) ? 1 : banner.banner

            // Failed
            if (rate < 1)
                addOrMerge({
                    pity: currentPity,
                    guaranteed: sim.guaranteed,
                    guaranteedPity: sim.guaranteedPity,
                    const: sim.const,
                    rate: sim.rate * (1 - rate)
                })

            // Got wanted banner item
            addOrMerge({
                pity: 0,
                guaranteed: false,
                guaranteedPity: 0,
                const: sim.const + 1,
                rate: sim.rate * rate * bannerRate * banner.guaranteed
            })

            // Got banner item but not wanted (eg. wrong rate up 4* char/5* char)
            if (banner.guaranteed < 1)
                if (banner.guaranteedPity && sim.guaranteedPity >= banner.guaranteedPity - 1)
                    // https://www.hoyolab.com/article/533196
                    addOrMerge({
                        pity: 0,
                        guaranteed: false,
                        guaranteedPity: 0,
                        const: sim.const + 1,
                        rate: sim.rate * rate * bannerRate * (1 - banner.guaranteed)
                    })
                else
                    addOrMerge({
                        pity: 0,
                        guaranteed: false,
                        guaranteedPity: banner.guaranteedPity ? sim.guaranteedPity + 1 : 0,
                        const: sim.const,
                        rate: sim.rate * rate * bannerRate * (1 - banner.guaranteed)
                    })

            // Failed banner items (eg. 4* char rate ups vs regular 4*)
            if (bannerRate < 1)
                addOrMerge({
                    pity: 0,
                    guaranteed: true,
                    guaranteedPity: banner.guaranteedPity ? sim.guaranteedPity + 1 : 0,
                    const: sim.const,
                    rate: sim.rate * rate * (1 - bannerRate)
                })
        }

        sims = Object.values(newSims)
        allSims.push(sims)
    }
    return allSims
}

const calcs = calcSimsRegular(-1, 0, 90, false, 0, gachas.char)
console.log(calcs)
