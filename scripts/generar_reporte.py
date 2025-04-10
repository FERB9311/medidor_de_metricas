import matplotlib.pyplot as plt
import pandas as pd
import os
from datetime import datetime
from fpdf import FPDF
import textwrap

class PDF(FPDF):
    def header(self):
        self.set_font('Arial', 'B', 12)
        self.cell(0, 10, 'Reporte Completo de Monitoreo del Sistema', 0, 1, 'C')
        self.ln(5)

    def footer(self):
        self.set_y(-15)
        self.set_font('Arial', 'I', 8)
        self.cell(0, 10, f'Página {self.page_no()}', 0, 0, 'C')

    def add_section_title(self, title):
        self.set_font('Arial', 'B', 12)
        self.cell(0, 10, title, 0, 1)
        self.ln(2)

    def add_paragraph(self, text):
        self.set_font('Arial', '', 10)
        for line in textwrap.wrap(text, width=100):
            self.cell(0, 6, line, 0, 1)
        self.ln(3)

def generar_reporte_completo():
    # Configuración inicial
    fecha_hora = datetime.now().strftime("%Y-%m-%d_%H-%M-%S")
    directorio_reportes = "reportes/"
    os.makedirs(directorio_reportes, exist_ok=True)
    
    try:
        # Leer datos del CSV
        df = pd.read_csv('data/historial.csv')
        
        # Crear PDF
        pdf = PDF()
        pdf.add_page()
        
        # 1. Información general
        pdf.add_section_title("Información General")
        pdf.add_paragraph(f"Período monitoreado: Desde {df['timestamp'].iloc[0]} hasta {df['timestamp'].iloc[-1]}")
        pdf.add_paragraph(f"Total de registros: {len(df)}")
        
        # 2. Gráficas principales
        pdf.add_section_title("Métricas Principales")
        
        # Gráfica de CPU
        plt.figure(figsize=(10, 4))
        plt.plot(df['timestamp'], df['cpu_total'], label='CPU Total (%)', color='blue')
        plt.title('Uso de CPU Total')
        plt.xlabel('Tiempo')
        plt.ylabel('Uso (%)')
        plt.xticks(rotation=45)
        plt.grid(True, linestyle='--', alpha=0.7)
        plt.tight_layout()
        img_cpu = "temp_cpu.png"
        plt.savefig(img_cpu, dpi=100)
        plt.close()
        pdf.image(img_cpu, x=10, w=190)
        os.remove(img_cpu)
        
        # 3. Tabla de estadísticas
        pdf.add_section_title("Estadísticas Clave")
        
        # Estadísticas de CPU
        stats_cpu = {
            'Métrica': ['Máximo', 'Mínimo', 'Promedio'],
            'CPU Total (%)': [
                df['cpu_total'].max(),
                df['cpu_total'].min(),
                df['cpu_total'].mean()
            ]
        }
        
        # Crear tabla
        pdf.set_font('Arial', 'B', 10)
        pdf.cell(40, 10, 'Métrica', 1)
        pdf.cell(40, 10, 'CPU Total (%)', 1)
        pdf.ln()
        
        pdf.set_font('Arial', '', 10)
        for i in range(3):
            pdf.cell(40, 10, stats_cpu['Métrica'][i], 1)
            pdf.cell(40, 10, f"{stats_cpu['CPU Total (%)'][i]:.1f}", 1)
            pdf.ln()
        
        # 4. Detalle de núcleos
        pdf.add_page()
        pdf.add_section_title("Uso por Núcleos de CPU")
        
        # Gráfica de núcleos (mostrar solo primeros 4 para claridad)
        nucleos = [col for col in df.columns if col.startswith('core_')]
        if nucleos:
            plt.figure(figsize=(10, 6))
            for nucleo in nucleos[:4]:  # Mostrar máximo 4 núcleos
                plt.plot(df['timestamp'], df[nucleo], label=nucleo)
            plt.title('Uso por Núcleo de CPU')
            plt.xlabel('Tiempo')
            plt.ylabel('Uso (%)')
            plt.legend()
            plt.xticks(rotation=45)
            plt.grid(True, linestyle='--', alpha=0.7)
            plt.tight_layout()
            img_nucleos = "temp_nucleos.png"
            plt.savefig(img_nucleos, dpi=100)
            plt.close()
            pdf.image(img_nucleos, x=10, w=190)
            os.remove(img_nucleos)
        
        # 5. Procesos principales
        pdf.add_section_title("Procesos con Mayor Consumo")
        procesos_cols = [col for col in df.columns if 'proc_' in col and 'nombre' in col]
        if procesos_cols:
            ultimo_registro = df.iloc[-1]
            pdf.set_font('Arial', 'B', 10)
            pdf.cell(70, 10, 'Proceso', 1)
            pdf.cell(30, 10, 'Uso CPU (%)', 1)
            pdf.ln()
            
            pdf.set_font('Arial', '', 10)
            for proc_col in procesos_cols[:5]:  # Mostrar máximo 5 procesos
                num = proc_col.split('_')[1]
                nombre = ultimo_registro[proc_col]
                uso = ultimo_registro[f'proc_{num}_cpu%']
                if nombre != 'N/A' and uso != 0:
                    pdf.cell(70, 10, nombre, 1)
                    pdf.cell(30, 10, str(uso), 1)
                    pdf.ln()
        
        # 6. Memoria y Swap
        pdf.add_page()
        pdf.add_section_title("Uso de Memoria y Swap")
        
        if 'mem_usada_MB' in df.columns:
            plt.figure(figsize=(10, 4))
            plt.plot(df['timestamp'], df['mem_usada_MB'], label='Memoria Usada (MB)', color='green')
            if 'mem_libre_MB' in df.columns:
                plt.plot(df['timestamp'], df['mem_libre_MB'], label='Memoria Libre (MB)', color='blue')
            plt.title('Uso de Memoria')
            plt.xlabel('Tiempo')
            plt.ylabel('MB')
            plt.legend()
            plt.xticks(rotation=45)
            plt.grid(True, linestyle='--', alpha=0.7)
            plt.tight_layout()
            img_mem = "temp_mem.png"
            plt.savefig(img_mem, dpi=100)
            plt.close()
            pdf.image(img_mem, x=10, w=190)
            os.remove(img_mem)
        
        # 7. Datos de disco (si existen)
        disk_cols = [col for col in df.columns if 'disk_' in col]
        if disk_cols:
            pdf.add_section_title("Información de Disco")
            pdf.set_font('Arial', '', 10)
            ultimo_disco = df.iloc[-1]
            for col in disk_cols:
                pdf.cell(0, 6, f"{col}: {ultimo_disco[col]}", 0, 1)
        
        # Guardar PDF
        pdf_path = os.path.join(directorio_reportes, f"reporte_completo_{fecha_hora}.pdf")
        pdf.output(pdf_path)
        print(f"Reporte completo generado: {pdf_path}")
        
    except Exception as e:
        print(f"Error al generar reporte: {str(e)}")

if __name__ == "__main__":
    generar_reporte_completo()