package com.yyy.game;

import com.yyy.engine.items.GameItem;
import com.yyy.engine.IHud;
import com.yyy.engine.items.TextItem;
import com.yyy.engine.Window;
import com.yyy.engine.graph.FontTexture;
import com.yyy.engine.graph.Material;
import com.yyy.engine.graph.Mesh;
import com.yyy.engine.graph.OBJLoader;
import org.joml.Vector4f;

import java.awt.*;

public class Hud implements IHud {

    private static final int FONT_COLS = 16;

    private static final int FONT_ROWS = 16;

    private static final String FONT_TEXTURE = "/textures/font_texture.png";

    private final GameItem[] gameItems;

    private final TextItem statusTextItem;

    private final GameItem compassItem;

    public Hud(String statusText) throws Exception {
        FontTexture fontTexture = new FontTexture(new Font("Purisa", Font.PLAIN, 24),"ISO-8859-1");
        this.statusTextItem = new TextItem(statusText,fontTexture);
        this.statusTextItem.getMesh().getMaterial().setAmbientColour(new Vector4f(1, 1, 1, 1));

        // Create compass
        Mesh mesh = OBJLoader.loadMesh("/models/compass.obj");
        Material material = new Material();
        material.setAmbientColour(new Vector4f(1, 0, 0, 1));
        mesh.setMaterial(material);
        compassItem = new GameItem(mesh);
        compassItem.setScale(40.0f);
        // Rotate to transform it to screen coordinates
        compassItem.setRotation(0f, 0f, 180f);

        gameItems = new GameItem[]{statusTextItem,compassItem};
    }

    public void setStatusText(String statusText) {
        this.statusTextItem.setText(statusText);
    }

    public void rotateCompass(float angle) {
        this.compassItem.setRotation(0, 0, 180 + angle);
    }

    @Override
    public GameItem[] getGameItems() {
        return gameItems;
    }

    public void updateSize(Window window) {
        this.statusTextItem.setPosition(10f, window.getHeight() - 50f, 0);
        this.compassItem.setPosition(window.getWidth() - 40f, 50f, 0);
    }
}
