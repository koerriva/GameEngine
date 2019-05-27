package com.yyy.game;

import com.yyy.engine.GameEngine;
import com.yyy.engine.IGameLogic;

public class Main {
    public static void main(String[] args) {
        try {
            boolean vSync = true;
            IGameLogic gameLogic = new DummyGame();
            GameEngine gameEng = new GameEngine("GAME", 1280, 720, vSync, gameLogic);
            gameEng.start();
        } catch (Exception excp) {
            excp.printStackTrace();
            System.exit(-1);
        }
    }
}
