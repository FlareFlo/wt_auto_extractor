#!/bin/sh
echo "Init"

dotnet /DepotDownloader/DepotDownloader.dll -app 236390 -depot 236391 -manifest 6590543165547691358 -filelist "list.txt"