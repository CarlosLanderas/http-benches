using System;
using System.IO;
using System.Linq;
using System.Net;
using System.Net.Http;
using System.Text.Json;
using System.Text.Json.Serialization;
using System.Threading;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Http;

using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;

namespace aspnetcore
{
    public class Program
    {
        public static void Main(string[] args)
        {
            CreateHostBuilder(args).Build().Run();
        }

        public static IHostBuilder CreateHostBuilder(string[] args) =>
            new HostBuilder().ConfigureWebHost(config =>
            {
                config.UseStartup<Startup>()
                    .UseKestrel(config => config.Listen(IPAddress.Loopback, 8080));
            });
    }
    
    public class Startup
    {
        public void ConfigureServices(IServiceCollection services) => services.AddRouting();
        
        public void Configure(IApplicationBuilder app, IWebHostEnvironment env)
        {
            var counter = 1;
            var mux = new Mutex();
            var contentType = "application/json";
            var url = "https://pokeapi.co/api/v2/ability";
            var client = new HttpClient();
            var jsonOptions = new JsonSerializerOptions()
            {
                PropertyNamingPolicy = JsonNamingPolicy.CamelCase
            };

            app.UseRouting();

            app.UseEndpoints(endpoints =>
            {
                endpoints.MapPost("/call", async context =>
                {
                    int abilityId;
                    mux.WaitOne();
                    abilityId = counter;
                    counter++;
                    mux.ReleaseMutex();

                    var stream = await client.GetStreamAsync($"{url}/{abilityId}");                   
                    var ability = await JsonSerializer.DeserializeAsync<Ability>(stream);
                    
                    using var ms2 = new MemoryStream();
                    var json = JsonSerializer.SerializeToUtf8Bytes<Ability>(ability, jsonOptions);
                    
                    context.Response.ContentType = contentType;
                    await context.Response.BodyWriter.WriteAsync(json);
                });
            });
        }
    }
    
    public class Ability
    { 
      [JsonPropertyName("id")]
      public int Id { get; set; }
      [JsonPropertyName("name")]
      public string Name { get; set; }
      [JsonPropertyName("is_main_series")]
      public bool IsMainSeries { get; set; }
    }
}