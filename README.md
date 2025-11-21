# Lab5-graficas
# 🌟 Estrella Animada con FBM (Renderizador por Software)

Este proyecto es un renderizador gráfico por software escrito en Rust que simula una estrella solar dinámica y burbujeante.

Utiliza un pipeline de gráficos personalizado para aplicar efectos de sombreado complejos, logrando una apariencia de turbulencia y alta emisión de calor.

# ✨ Características Clave

Turbulencia (FBM): Se usa Ruido de Valor Fractal (FBM) en 3D para simular la actividad y las manchas solares que se mueven con el tiempo.

Distorsión de Vértices: El Vertex Shader deforma ligeramente la geometría de la estrella para crear un efecto de "flare" (erupción) pulsante.

Gradiente Dinámico: El color de la superficie cambia según la intensidad del ruido, simulando las diferentes temperaturas (rojo, naranja, amarillo y blanco).

# Cómo Clonar y Ejecutar

Para poner en marcha este proyecto, necesitas tener Rust y Cargo instalados.

1. Clonar el Repositorio

Abre tu terminal y ejecuta el siguiente comando para descargar el código fuente:

git clone https://github.com/Qu3zada22/Lab5-graficas.git


<img width="966" height="604" alt="imagen" src="https://github.com/user-attachments/assets/7c04a099-2923-40d7-bd03-7425bc35e4f4" />



2. Ejecutar el Proyecto

Navega al directorio del proyecto y usa Cargo para compilar y ejecutar:

cd lab5graph
cargo run
