import random

# Definimos las dimensiones del mapa
width = 50
height = 30

# Definimos la probabilidad de que una celda sea suelo (fondo verde)
ground_probability = 0.4

# Generamos el mapa hasta que la posición inicial del jugador esté en una celda de suelo
player_x = None
player_y = None
while (
    (player_x is None)
    or (not (0 <= player_x < width))
    or (not (0 <= player_y < height))
    or (map_data[player_y * (width + 1) + player_x] != " ")
):
    map_data = ""
    for y in range(height):
        for x in range(width):
            if random.random() < ground_probability:
                map_data += "\033[42m \033[0m"
            else:
                map_data += "\033[44m \033[0m"
        map_data += "\n"

    # Escogemos una posición aleatoria para el jugador
    player_x = random.randint(0, width - 1)
    player_y = random.randint(0, height - 1)

# Imprimimos el mapa con el jugador en su posición inicial
map_data_with_player = list(map_data)
map_data_with_player[player_y * (width + 1) + player_x] = "\033[31m@\033[0m"
map_data_with_player = "".join(map_data_with_player)
print(map_data_with_player)

# Movemos al jugador hacia la derecha
player_x += 1

map_data_with_player = list(map_data)
map_data_with_player[player_y * (width + 1) + player_x] = "\033[31m@\033[0m"
map_data_with_player = "".join(map_data_with_player)
print(map_data_with_player)
