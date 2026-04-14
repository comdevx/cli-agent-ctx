#!/bin/sh
# scripts/install.sh — shell installer for agent-ctx
# Usage: curl -fsSL https://comdevx.github.io/cli-agent-ctx/install.sh | sh
set -e

TOOL_NAME="agent-ctx"
REPO="comdevx/cli-agent-ctx"
INSTALL_DIR="${HOME}/.local/bin"

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     OS=unknown-linux-gnu;;
    Darwin*)    OS=apple-darwin;;
    MINGW*|MSYS*|CYGWIN*) OS=pc-windows-msvc;;
    *)          echo "error: unsupported OS: ${OS}"; exit 1;;
esac

# Detect architecture
ARCH="$(uname -m)"
case "${ARCH}" in
    x86_64|amd64)   ARCH=x86_64;;
    aarch64|arm64)   ARCH=aarch64;;
    *)               echo "error: unsupported architecture: ${ARCH}"; exit 1;;
esac

TARGET="${ARCH}-${OS}"

# Get latest version
VERSION=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
    | grep '"tag_name"' | sed -E 's/.*"v([^"]+)".*/\1/')

if [ -z "${VERSION}" ]; then
    echo "error: could not determine latest version"
    exit 1
fi

# Determine archive extension
if [ "${OS}" = "pc-windows-msvc" ]; then
    EXT="zip"
else
    EXT="tar.gz"
fi

ARCHIVE="${TOOL_NAME}-v${VERSION}-${TARGET}.${EXT}"
URL="https://github.com/${REPO}/releases/download/v${VERSION}/${ARCHIVE}"

echo "Installing ${TOOL_NAME} v${VERSION} for ${TARGET}..."

TMPDIR=$(mktemp -d)
trap 'rm -rf "${TMPDIR}"' EXIT

curl -fsSL "${URL}" -o "${TMPDIR}/${ARCHIVE}"

# Extract
if [ "${EXT}" = "zip" ]; then
    unzip -q "${TMPDIR}/${ARCHIVE}" -d "${TMPDIR}"
else
    tar -xzf "${TMPDIR}/${ARCHIVE}" -C "${TMPDIR}"
fi

# Install
mkdir -p "${INSTALL_DIR}"
mv "${TMPDIR}/${TOOL_NAME}" "${INSTALL_DIR}/${TOOL_NAME}"
chmod +x "${INSTALL_DIR}/${TOOL_NAME}"

echo ""
echo "${TOOL_NAME} v${VERSION} installed to ${INSTALL_DIR}/${TOOL_NAME}"
echo ""

# Check if in PATH
if ! echo "${PATH}" | grep -q "${INSTALL_DIR}"; then
    echo "Add to your PATH:"
    echo "  export PATH=\"\${HOME}/.local/bin:\${PATH}\""
    echo ""
fi

echo "Get started:"
echo "  ${TOOL_NAME} --help"
echo "  ${TOOL_NAME} init"
