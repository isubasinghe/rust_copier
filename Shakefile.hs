import Development.Shake
import Development.Shake.Command
import Development.Shake.FilePath
import Development.Shake.Util
import System.Console.GetOpt

newtype Flags = Platform String
  deriving (Show, Eq)

flags :: [OptDescr (Either String Flags)]
flags = [Option "p" ["platform"] (ReqArg (Right . Platform) "PLAT") "Platform to build for"]

getPlatform :: [Flags] -> String
getPlatform [] = error "A platform is a requirement but none was given"
getPlatform (x : xs) = case x of
  Platform s -> s

withPlatform :: [Flags] -> (String -> Rules ()) -> Rules ()
withPlatform flags fn = let platform = getPlatform flags in fn platform

main :: IO ()
main = shakeArgsWith
  ( shakeOptions
      { shakeFiles = "_build",
        shakeColor = True,
        shakeStorageLog = True,
        shakeTimings = True,
        shakeThreads = 0,
        shakeProgress = progressSimple
      }
  )
  flags
  $ \flags targets -> pure $
    Just $
      withPlatform flags $ \platform -> do
        want (if targets == ["clean"] then targets else targets ++ ["_build/os.elf"])

        phony "clean" $ do
          putInfo "Cleaning files in build"
          removeFilesAfter "_build" ["//*"]

        "_build/libmukernel.a" %> \out -> do
          rs <- getDirectoryFiles "" ["//*.rs"]
          need rs
          cmd_ "cargo build --release" "--features" platform "--target" ("src/plat/" ++ platform ++ "/rust.json")
          cmd_ "cp ./target/rust/release/libmukernel.a" out

        "_build/os.elf" %> \out -> do
          putInfo "Making ELF file"
          need ["_build/libmukernel.a"]
          asms <- getDirectoryFiles "" ["src/plat/" ++ platform ++ "//*.s"]
          need asms
          cmd_ "riscv64-unknown-linux-gnu-gcc -static -ffreestanding -nostdlib -fno-exceptions -march=rv64gc -mabi=lp64" ("-Tsrc/plat/" ++ platform ++ "/board.ld") "-o" out asms "-L./target/rust/release" "-lmukernel"
