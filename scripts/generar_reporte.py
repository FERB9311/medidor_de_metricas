import matplotlib.pyplot as plt
import pandas as pd
import os
from datetime import datetime

# Obtener la fecha y hora actuales
fecha_hora = datetime.now().strftime("%Y%m%d_%H%M%S")

# Directorio donde se guardan los reportes
directorio_reportes = "reporte/"

# Crear el directorio si no existe
os.makedirs(directorio_reportes, exist_ok=True)

# Datos de ejemplo para el reporte (puedes adaptarlo a lo que uses)
cpu_data = [20, 35, 50, 65, 80]  # Aquí puedes poner tus datos reales
tiempos = ['10:00', '10:10', '10:20', '10:30', '10:40']

# Gráfica de uso de CPU
plt.plot(tiempos, cpu_data, label='Uso de CPU')
plt.xlabel('Tiempo')
plt.ylabel('Uso (%)')
plt.title('Uso de CPU durante el monitoreo')
plt.legend()

# Guardar la gráfica con fecha y hora en el nombre
plt.savefig(os.path.join(directorio_reportes, f"cpu_total_{fecha_hora}.png"))
plt.close()

# Resumen del reporte en un archivo de texto
resumen = f"Reporte generado el {fecha_hora}\n\nUso de CPU: {cpu_data}\nTiempos: {tiempos}"

# Guardar el resumen
with open(os.path.join(directorio_reportes, f"resumen_{fecha_hora}.txt"), "w") as archivo:
    archivo.write(resumen)

print(f"Reporte generado: {directorio_reportes}cpu_total_{fecha_hora}.png")
print(f"Resumen generado: {directorio_reportes}resumen_{fecha_hora}.txt")
