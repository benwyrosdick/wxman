class Wxman < Formula
  desc "A terminal-based weather application"
  homepage "https://github.com/benwyrosdick/wxman"
  version "0.1.3"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/benwyrosdick/wxman/releases/download/v#{version}/wxman-aarch64-apple-darwin.tar.gz"
      sha256 "SHA256_ARM_DARWIN"
    else
      url "https://github.com/benwyrosdick/wxman/releases/download/v#{version}/wxman-x86_64-apple-darwin.tar.gz"
      sha256 "SHA256_X86_DARWIN"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      odie "wxman is not currently supported on Linux ARM"
    else
      url "https://github.com/benwyrosdick/wxman/releases/download/v#{version}/wxman-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "SHA256_LINUX"
    end
  end

  def install
    bin.install "wxman"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/wxman --version")
  end
end
