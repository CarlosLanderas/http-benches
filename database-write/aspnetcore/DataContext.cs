using aspnetcore.Properties;
using Microsoft.EntityFrameworkCore;

namespace aspnetcore
{
    public class DataContext: DbContext
    {
        public DataContext(DbContextOptions options): base(options){}
        
        public DbSet<User> Users { get; set; }

        protected override void OnModelCreating(ModelBuilder modelBuilder)
        {

            var user = modelBuilder.Entity<User>();
            user.HasKey(u => u.Id);
            user.Property(p => p.Id).ValueGeneratedOnAdd();
            user.Property(p => p.Email).HasMaxLength(50).IsRequired();
            user.Property(p => p.Email).IsRequired();
        }
    }
    
}