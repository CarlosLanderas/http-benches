using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Net;
using System.Text.Json;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Builder;
using Microsoft.AspNetCore.Hosting;
using Microsoft.AspNetCore.Http;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using MongoDB.Bson;
using MongoDB.Bson.Serialization.Attributes;
using MongoDB.Driver;

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
                var mongoClient  = new MongoClient("mongodb://root:example@localhost:27017");
                var books = mongoClient.GetDatabase("main").GetCollection<Book>("books");
                var contentType = "application/json";

                var jsonOptions = new JsonSerializerOptions
                {
                    PropertyNamingPolicy = JsonNamingPolicy.CamelCase
                };
                
                endpoints.MapPost("/books", async context =>
                {
                    var id = Guid.NewGuid().ToString();
                    var book = new Book {Name = $"Book{id}", Sn = id};
                    await books.InsertOneAsync(book);
                    await context.Response.WriteAsync(id);
                });

                endpoints.MapGet("/books", async context =>
                {
                    var results = await books.FindAsync(b => true);
                    using var ms = new MemoryStream();
                    await JsonSerializer.SerializeAsync(ms, results.ToList(), jsonOptions);
                    ms.Position = 0;
                    context.Response.ContentType = contentType;
                    await context.Response.BodyWriter.WriteAsync(ms.ToArray());
                });
            });
        }
    }
    
    public class Book
    {
        [BsonId]
        [BsonRepresentation(BsonType.ObjectId)]
        public string Id { get; set; }
        [BsonElement("Name")]
        public string Name { get; set; }
        public string Sn { get; set; }
    }
}