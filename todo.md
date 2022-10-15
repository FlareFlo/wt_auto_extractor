
1. Make steam API / steamdb calls to know when a new game version is available (~~maybe too few compare to launcher subversions?~~ seems they update in-sync with minor patches)
2. Extract bin from base folder into git-repo
3. ~~Make steamcmd wrapper that automatically updates game with auth~~
   1. Use [this](https://github.com/SteamRE/DepotDownloader) from [here](https://steamdb.info/app/236390/depots/) as it seems to do the job better, with less throughput and dev-server being available
   ```sh
   DepotDownloader.exe -app 236390 -depot 236391 -manifest 6590543165547691358 -filelist "list.txt" -username $USR -password $PASSWD
   ```
4. Build proper docker env to package process chain better