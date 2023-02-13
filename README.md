## Pielectricseagles-usb-update-desktop 

A tools that create update image to USB drive and for updating and add files to drone with secure transfet file. 

Help to reduce confuisng what file is binary actual version. 

You copy file to usb drive with creating protecting of confusing file, even file with exact same name. 

Checksum is help to recognise version file.

## Installation

You require install cargo package XCompress before usage this software. You can download from release repository https://github.com/magiclen/xcompress/releases and manually copy to 
```
/usr/local/bin/
```

Example install and test xcompress

```
sudo cp xcompress /usr/local/bin/xcompress
sudo chmod -R 777 /usr/local/bin/xcompress
nano file.txt
xcompress a -p 1234 file.txt -o file.7z
```

Other way, you can install via cargo command if have install cargo and rust programming language compilier in your Linux PC. 

```
cargo install xcompress
```

You can download binary from release. You will need to set premission for this binary in your linux PC. Just enter a command before usage

```
sudo chmod -R 777 drone_image_generator
```

## Usage this CLI application 

Example of usage

```
./drone_image_generator --files-to-install=/home/linuxpc/Desktop/DroneImageGenerator/pielectricseagles --install-path=/usr/bin/ --files-to-install=/home/linuxpc/Desktop/DroneImageGenerator/config.ini --install-path=/etc/pielectricseagles/ --post-update-script-content-file=/home/linuxpc/Desktop/DroneImageGenerator/after.sh --pre-update-script-content-file=/home/linuxpc/Desktop/DroneImageGenerator/before.sh --drive=/media/linuxpc/DRONE --password=12345678
```

``` drone_image_generator ``` is a binary file that will run this CLI application
```--files-to-install= ``` is a source file path from PC that it will install in Raspberry Pi Zero 
```--install-path=``` is a destination file path installation in Raspberry Pi Zero 
```--post-update-script-content-file=``` is a script after installation file in Raspberry Pi Zero
```--pre-update-script-content-file=``` is a script before installation file Raspberry Pi Zero
```--drive=``` is a path of USB drive that will make a image to specific USB drive
```--password=``` is a password installation zip file for identity device and vendor

## Troubleshooting

If you got error about file and directory paths, please check carefully that files are existed. (Will show RUST panic inforamtion after early ternimate this tool) 
