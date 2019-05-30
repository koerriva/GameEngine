package com.yyy.engine;

import com.yyy.engine.items.GameItem;

public interface IHud {
    GameItem[] getGameItems();

    default void cleanup() {
        GameItem[] gameItems = getGameItems();
        for (GameItem gameItem : gameItems) {
            gameItem.getMesh().cleanUp();
        }
    }
}
