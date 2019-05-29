package com.yyy.game;

import com.yyy.engine.GameItem;
import com.yyy.engine.Utils;
import com.yyy.engine.Window;
import com.yyy.engine.graph.*;
import org.joml.Matrix4f;
import org.joml.Vector3f;
import org.joml.Vector4f;

import static org.lwjgl.opengl.GL11.*;

public class Renderer {
    private ShaderProgram shaderProgram;
    /**
     * Field of View in Radians
     */
    private static final float FOV = (float) Math.toRadians(60.0f);

    private static final float Z_NEAR = 0.01f;

    private static final float Z_FAR = 1000.f;

    private final Transformation transformation;

    private float specularPower;

    public Renderer() {
        transformation = new Transformation();
        specularPower = 10f;
    }

    public void init(Window window) throws Exception {
        shaderProgram = new ShaderProgram();
        shaderProgram.createVertexShader(Utils.loadResource("/shaders/vertex.vert"));
        shaderProgram.createFragmentShader(Utils.loadResource("/shaders/fragment.frag"));
        shaderProgram.link();

        // Create projection matrix
        float aspectRatio = (float) window.getWidth() / window.getHeight();
        shaderProgram.createUniform("projectionMatrix");
        shaderProgram.createUniform("modelViewMatrix");
        shaderProgram.createUniform("texture_sampler");
        // Create uniform for material
        shaderProgram.createMaterialUniform("material");
        // Create lighting related uniforms
        shaderProgram.createUniform("specularPower");
        shaderProgram.createUniform("ambientLight");
        shaderProgram.createPointLightUniform("pointLight");

        window.setClearColor(0.0f, 0.0f, 0.0f, 0.0f);
    }

    public void clear() {
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    }

    public void render(Window window, Camera camera, Vector3f ambientLight, PointLight pointLight, GameItem... items) {
        clear();

        if (window.isResized()) {
            glViewport(0, 0, window.getWidth(), window.getHeight());
            window.setResized(false);
        }

        shaderProgram.bind();
        // Update projection Matrix
        Matrix4f projectionMatrix = transformation.getProjectionMatrix(FOV, window.getWidth(), window.getHeight(), Z_NEAR, Z_FAR);
        shaderProgram.setUniform("projectionMatrix", projectionMatrix);

        // Update view Matrix
        Matrix4f viewMatrix = transformation.getViewMatrix(camera);

        // Update Light Uniforms
        shaderProgram.setUniform("ambientLight", ambientLight);
        shaderProgram.setUniform("specularPower", specularPower);
        // Get a copy of the light object and transform its position to view coordinates
        PointLight currPointLight = new PointLight(pointLight);
        Vector3f lightPos = currPointLight.getPosition();
        Vector4f aux = new Vector4f(lightPos, 1);
        aux.mul(viewMatrix);
        lightPos.x = aux.x;
        lightPos.y = aux.y;
        lightPos.z = aux.z;
        shaderProgram.setUniform("pointLight", currPointLight);

        shaderProgram.setUniform("texture_sampler", 0);
        // Render each gameItem
        // Update view Matrix
        for (GameItem gameItem:items){
            // Set world matrix for this item
            Matrix4f modelViewMatrix =
                    transformation.getModelViewMatrix(gameItem,viewMatrix);
            shaderProgram.setUniform("modelViewMatrix", modelViewMatrix);
            // Render the mes for this game item
            Mesh mesh = gameItem.getMesh();
            shaderProgram.setUniform("material",mesh.getMaterial());
            // Render the mesh for this game item
            gameItem.getMesh().render();
        }

        shaderProgram.unbind();
    }

    public void cleanup() {
        if (shaderProgram != null) {
            shaderProgram.cleanup();
        }
    }
}
