class DummyGame:public IGameLogic
{
private:
    /* data */
public:
    DummyGame();
    ~DummyGame();

    void init(){}

    void input(Window* window){}

    void update(float interval){}

    void render(Window* window){}
};

DummyGame::DummyGame(/* args */){}

DummyGame::~DummyGame(){}