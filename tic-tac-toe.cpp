#include <iostream>
#include <string>

void input(std::string MoveX, std::string MoveY, int board[3][3], int Counter) {

        std::cout << "Input space for X: ";
        std::cin >> MoveX;
        std::cout << "Input space for Y: ";
        std::cin >> MoveY;

        if (board[stoi(MoveY)][stoi(MoveX)] != 0) {

		std::cout << "Not an empty space" << std::endl;

                input(MoveX, MoveY, board, Counter);

        }

        else {

                 board[stoi(MoveY)][stoi(MoveX)] = Counter;

	}

}

int main() {

	bool gameOver = false;

	int moveCounter = 1;

	int board[3][3] = {{0, 0, 0}, {0, 0, 0}, {0, 0, 0}};

	std::string currentMoveX;
	std::string currentMoveY;

	while (gameOver == false) {

		if (moveCounter == 1) {

			std::cout << "X's turn" << std::endl;

			input(currentMoveX, currentMoveY, board, moveCounter);

			moveCounter = 2;

		}

		else if (moveCounter == 2) {

			std::cout << "O's turn" << std::endl;

			input(currentMoveX, currentMoveY, board, moveCounter);

			moveCounter = 1;

		}

		for (int i = 0; i < 3; i++) {

			for (int j = 0; j < 3; j++) {

				if (board[i][j] == 0) {
				
					std::cout << "_ ";

				}

				if (board[i][j] == 1) {
			
					std::cout << "X ";
			
				}

				if (board[i][j] == 2) {

					std::cout << "O ";

				}

			}

			std::cout << std::endl;

		}

	}

	return 0;
}
