FROM mcr.microsoft.com/dotnet/sdk:9.0 AS build
RUN apt-get update
RUN apt-get -yqq install clang zlib1g-dev

WORKDIR /app
COPY src .
#RUN dotnet publish appMpower/appMpower.csproj -c Release -o out
RUN dotnet publish -c Release -o out

# Construct the actual image that will run
FROM mcr.microsoft.com/dotnet/aspnet:9.0 AS runtime
# Full PGO
ENV DOTNET_TieredPGO 1 
ENV DOTNET_TC_QuickJitForLoops 1 
ENV DOTNET_ReadyToRun 0

ENV ASPNETCORE_URLS http://+:8080
WORKDIR /app
COPY --from=build /app/out ./

EXPOSE 8080

ENTRYPOINT ["./appMpower"]