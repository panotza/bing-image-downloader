# Bing Image Downloader
<table>
<tr>
<td>
  A simple program to download Bing daily image to given directory written in rust.
  
  This is my learning rust project so it may contain bugs and bad codes.
</td>
</tr>
</table>

## Usage 
- Clone Project and run `Cargo build` or Download Prebuild binary
- Run `bing-image-downloader image_dir size`

## Example
```bash
  bing-image-downloader "/home/user" UHD
```
on Windows you can use Task Scheduleder to regulary download image and set as wallpaper slide show
```bash
SCHTASKS /CREATE /SC HOURLY /TN "FOLDERPATH\TASKNAME" /TR "C:\SOURCE\FOLDER\APP-AND-ARGS"
```
