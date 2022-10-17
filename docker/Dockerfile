
# Core deps for running the rust-app and the C# downloader
FROM archlinux
RUN pacman -Syu --noconfirm rustup git base-devel dotnet-runtime wget
RUN pacman --noconfirm -Scc

## Full checkout and build of main-script
#RUN rustup default nightly
#RUN git clone https://github.com/FlareFlo/wt_auto_extractor.git
#WORKDIR ./wt_auto_extractor
#RUN cargo build
#RUN mv target/debug/wt_auto_extractor wt_auto_extractor
#RUN rm -rf target
#WORKDIR ..

# Download DepotDownloader
WORKDIR /DepotDownloader
ADD "https://github.com/SteamRE/DepotDownloader/releases/download/DepotDownloader_2.4.7/depotdownloader-2.4.7.zip" .
RUN bsdtar xvf "depotdownloader-2.4.7.zip"
WORKDIR ..

# Copy configuration files
COPY ./list.txt .
COPY ./gitauth .
COPY ./steam_usr .
COPY ./steam_pwd .

# Run the actual process
COPY ./run.sh .
CMD ["./run.sh"]
