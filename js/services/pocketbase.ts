// npm install pocketbase
import PocketBase from "pocketbase";
import { Config } from "../../crates/pkg/rusty_web_tetris";

const pb = new PocketBase("https://scores.alecdivito.com");

interface ScoreMetadata extends Config {
    metadata: {
        time: number
        rows: number
        level: number
    }
}

async function getTetris() {
    return pb.collection("games").getOne('18anr98mj1n6ugx')
}

export async function submitGameScore(player: string, score: number, metadata = {}) {
    const game = await getTetris();
    return pb.collection("scores").create({
        game: game.id,
        player,
        score,
        metadata,
    });
}

/**
 * Top N scores for a game.
 * sort: "-score" if higher is better, "score" if lower is better (e.g. time).
 */
export async function topScores(limit = 100, sort = "-score") {
    const game = await getTetris();
    return pb.collection("scores").getList(1, limit, {
        filter: `game = "${game.id}"`,
        sort,
    });
}
