# ==============================================================================
# SCRIPT DE SINCRONIZACIÓN: CARPETA WIKI -> GITHUB WIKI REPO
# ==============================================================================
# Sincroniza el contenido de 'wiki/' con el repositorio de la pestaña Wiki:
# https://github.com/caidevOficial/Yaguarete_Lang.wiki.git
#
# Utiliza un directorio temporal aislado para no interferir con el repositorio
# principal de Yaguarete_Lang en la rama main.
# ==============================================================================

param(
    [string]$RemoteUrl = "https://github.com/caidevOficial/Yaguarete_Lang.wiki.git",
    [string]$Branch = "master"
)

$ErrorActionPreference = "Stop"

$SourceDir = $PSScriptRoot
$TempDeployDir = Join-Path $env:TEMP ("yaguarete_wiki_deploy_" + [System.Guid]::NewGuid().ToString().Substring(0, 8))

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "  Sincronizador de la Wiki Oficial de YAGUARETÉ v0.1.1" -ForegroundColor Yellow
Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "Carpeta fuente local:  $SourceDir" -ForegroundColor Gray
Write-Host "Repositorio remoto:    $RemoteUrl" -ForegroundColor Gray
Write-Host "Directorio temporal:   $TempDeployDir" -ForegroundColor Gray
Write-Host ""

# 1. Comprobar Git
try {
    $gitVersion = git --version
    Write-Host "[OK] Git detectado: $gitVersion" -ForegroundColor Green
} catch {
    Write-Host "[ERROR] Git no está instalado o no se encuentra en el PATH." -ForegroundColor Red
    exit 1
}

# 2. Crear carpeta temporal y copiar archivos Markdown
try {
    New-Item -ItemType Directory -Path $TempDeployDir -Force | Out-Null
    
    # Copiar todos los archivos .md a la carpeta temporal
    Get-ChildItem -Path $SourceDir -Filter "*.md" | ForEach-Object {
        Copy-Item -Path $_.FullName -Destination $TempDeployDir -Force
    }
    
    Write-Host "[OK] Archivos Markdown preparados para la publicación." -ForegroundColor Green
} catch {
    Write-Host "[ERROR] No se pudo preparar la carpeta temporal: $_" -ForegroundColor Red
    exit 1
}

# 3. Inicializar repositorio aislado en la carpeta temporal
Push-Location $TempDeployDir
try {
    git init -b $Branch | Out-Null
    git remote add origin $RemoteUrl
    git add -A
    
    $commitMsg = "Docs: Sincronización oficial de la Wiki de Yaguarete v0.1.1"
    git commit -m $commitMsg | Out-Null
    Write-Host "[OK] Commit de Wiki generado correctamente en entorno aislado." -ForegroundColor Green
    
    Write-Host ""
    Write-Host "==========================================================" -ForegroundColor Cyan
    Write-Host "  LISTO PARA PUBLICAR EN GITHUB WIKI" -ForegroundColor Yellow
    Write-Host "==========================================================" -ForegroundColor Cyan
    Write-Host "El repositorio de la Wiki está listo para enviarse a:" -ForegroundColor White
    Write-Host "    $RemoteUrl (rama: $Branch)" -ForegroundColor Green
    Write-Host ""
    Write-Host "Aviso: Si la pestaña Wiki en GitHub es nueva, el primer push" -ForegroundColor Gray
    Write-Host "puede requerir el flag --force para establecer la raíz." -ForegroundColor Gray
    Write-Host "==========================================================" -ForegroundColor Cyan
    
    $respuesta = Read-Host "¿Deseas publicar la Wiki en GitHub ahora mismo? (s/n)"
    if ($respuesta -match '^[sS]') {
        Write-Host "[INFO] Enviando cambios a GitHub Wiki..." -ForegroundColor Cyan
        git push -u origin $Branch --force
        Write-Host ""
        Write-Host "[ÉXITO] ¡Wiki publicada con éxito en GitHub!" -ForegroundColor Green
        Write-Host "Visítala en: https://github.com/caidevOficial/Yaguarete_Lang/wiki" -ForegroundColor Cyan
    } else {
        Write-Host "[INFO] Publicación cancelada por el usuario." -ForegroundColor Yellow
    }
} catch {
    Write-Host "[ERROR] Ocurrió un error durante la sincronización: $_" -ForegroundColor Red
} finally {
    Pop-Location
    # Limpiar carpeta temporal
    Remove-Item -Path $TempDeployDir -Recurse -Force -ErrorAction SilentlyContinue
}
