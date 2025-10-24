#!/usr/bin/env python3

# Script Python que simula tree y genera URLs de GitHub
# No requiere tree externo - implementa su propia lógica de directorio
# Uso: python3 darwinx_standalone.py [directorio] [nivel]

import sys
import argparse
from pathlib import Path

GITHUB_BASE = "https://raw.githubusercontent.com/lzuccolo/darwinx/refs/heads/main/crates"

def is_file_type(path):
    """Verifica si es un archivo del tipo que nos interesa"""
    extensions = {'.rs', '.toml', '.proto', '.md', '.sql', '.yaml', '.yml', '.json', '.txt', '.lock'}
    return path.suffix in extensions

def generate_tree_structure(directory, max_level=4):
    """Genera la estructura de árbol de directorios"""
    directory = Path(directory)
    if not directory.exists():
        return []
    
    files_and_dirs = []
    
    def scan_directory(path, level=0, prefix=""):
        if level >= max_level:
            return
        
        try:
            # Obtener todos los elementos y ordenarlos
            items = sorted(path.iterdir(), key=lambda x: (x.is_file(), x.name))
            
            for i, item in enumerate(items):
                is_last = i == len(items) - 1
                
                # Crear prefijo para este elemento
                if level == 0:
                    current_prefix = "├── " if not is_last else "└── "
                    next_prefix = "│   " if not is_last else "    "
                else:
                    current_prefix = prefix + ("├── " if not is_last else "└── ")
                    next_prefix = prefix + ("│   " if not is_last else "    ")
                
                # Agregar elemento actual
                display_name = item.name
                if item.is_dir():
                    display_name += "/"
                
                files_and_dirs.append({
                    'level': level + 1,
                    'name': item.name,
                    'path': item,
                    'is_file': item.is_file(),
                    'display': current_prefix + display_name,
                    'relative_path': item.relative_to(directory)
                })
                
                # Recursión para subdirectorios
                if item.is_dir():
                    scan_directory(item, level + 1, next_prefix)
                    
        except PermissionError:
            pass
    
    scan_directory(directory)
    return files_and_dirs

def generate_github_urls(items, github_base, directory):
    """Genera URLs de GitHub desde los elementos del árbol"""
    urls = []
    base_dir = Path(directory).name
    
    for item in items:
        # Solo procesar archivos del tipo que nos interesa
        if item['is_file'] and is_file_type(item['path']):
            # Construir path relativo
            rel_path = str(item['relative_path'])
            
            # Limpiar path para GitHub si es necesario
            if base_dir != 'crates':
                # Si el directorio no es 'crates', no incluir su nombre en el path
                rel_path = rel_path
            
            urls.append(f"{github_base}/{rel_path}")
    
    return urls

def main():
    parser = argparse.ArgumentParser(
        description='Genera URLs de GitHub desde la estructura de directorios (sin tree externo)',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog='''
Ejemplos de uso:
  python3 darwinx_standalone.py ../crates
  python3 darwinx_standalone.py ../crates 3
  python3 darwinx_standalone.py . 4
        '''
    )
    
    parser.add_argument(
        'directory',
        nargs='?',
        default='../crates',
        help='Directorio a analizar (default: ../crates)'
    )
    
    parser.add_argument(
        'level',
        nargs='?',
        type=int,
        default=4,
        help='Nivel de profundidad (default: 4)'
    )
    
    parser.add_argument(
        '--base-url',
        default=GITHUB_BASE,
        help=f'URL base de GitHub (default: {GITHUB_BASE})'
    )
    
    parser.add_argument(
        '--show-tree',
        action='store_true',
        help='Mostrar estructura de árbol antes de las URLs'
    )
    
    parser.add_argument(
        '--stats',
        action='store_true',
        default=True,
        help='Mostrar estadísticas (default: True)'
    )
    
    args = parser.parse_args()
    
    # Verificar que el directorio existe
    directory = Path(args.directory)
    if not directory.exists():
        print(f"Error: El directorio '{args.directory}' no existe", file=sys.stderr)
        sys.exit(1)
    
    print(f"Analizando directorio: {directory} (nivel {args.level})", file=sys.stderr)
    
    # Generar estructura de árbol
    tree_items = generate_tree_structure(directory, args.level)
    
    if not tree_items:
        print("No se encontraron elementos en el directorio", file=sys.stderr)
        sys.exit(1)
    
    # Mostrar estructura de árbol si se solicita
    if args.show_tree:
        print(f"\n{directory.name}/")
        for item in tree_items:
            print(item['display'])
        print()
    
    # Generar URLs de GitHub
    urls = generate_github_urls(tree_items, args.base_url, args.directory)
    
    # Ordenar URLs
    urls.sort()
    
    # Imprimir URLs
    for url in urls:
        print(url)
    
    # Mostrar estadísticas
    if args.stats:
        total_files = sum(1 for item in tree_items if item['is_file'])
        processed_files = len(urls)
        print(f"# Total de archivos encontrados: {total_files}", file=sys.stderr)
        print(f"# Archivos procesados: {processed_files}", file=sys.stderr)
        print(f"# Directorio analizado: {args.directory}", file=sys.stderr)
        print(f"# Nivel de profundidad: {args.level}", file=sys.stderr)

if __name__ == '__main__':
    main()
