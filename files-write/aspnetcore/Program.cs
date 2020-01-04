using System;
using System.Net;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using System.IO;
using System.Linq;
using System.Text;
using Microsoft.AspNetCore.Http;

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
            app.UseRouting();

            app.UseEndpoints(endpoints =>
            {
                if (!Directory.Exists("files"))
                {
                    Console.WriteLine("Creating files directory");
                    Directory.CreateDirectory("files");
                }

                endpoints.MapPost("/file", async context =>
                {
                    var guid = Guid.NewGuid().ToString();
                    Memory<byte> contents = Encoding.UTF8.GetBytes("The content of the file is ");
                    
                    var index = 0;
                    
                    var fileContents = new Memory<byte>(new byte[contents.Length + guid.Length]);
                    contents.CopyTo(fileContents.Slice(index, contents.Length));
                    index += contents.Length;
                    
                    var guidBytes = Encoding.UTF8.GetBytes(guid);
                    guidBytes.CopyTo(fileContents.Slice(index, guid.Length));
                    
                    File.WriteAllBytes($"files/{guid}", fileContents.ToArray());
                    await context.Response.BodyWriter.WriteAsync(guidBytes);
                });
            });
        }
    }
}