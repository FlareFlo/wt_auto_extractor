#!/bin/sh
echo "Setting steam-auth"
PWD=$(cat steam_pwd)
USR=$(cat steam_usr)

dotnet /DepotDownloader/DepotDownloader.dll -app 236390 -depot 236391 -manifest 6590543165547691358 -filelist "list.txt" -username "$USR" -password "$PWD"
ls -a