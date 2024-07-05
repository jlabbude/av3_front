import "../../App.css";

function Home() {
  return (
    <>
      <div className='wrapper' style={{ width: "100%", height: "100vh"}}>
        <main style={{ display: "flex", flexDirection: "column", alignItems: 'center', backgroundImage: 'url("https://images.unsplash.com/photo-1473876637954-4b493d59fd97?q=80&w=1992&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D")', backgroundRepeat: "no-repeat", backgroundSize: "cover", height: "1300px", gap: "5rem" }}>
          <header style={{ display: "flex", justifyContent: "center", flexDirection: "column", alignItems: 'center' }}><h1>Centro de mapeamento de poluição</h1></header>
          <div style={{ display: 'flex', gap: "20rem" }}>
            <section className='image-wrapper' style={{ width: "700px", height: "760px" }}>
              <a href="http://localhost:8080/map" target="_blank" rel="noopener noreferrer">
                <img src="src/assets/map.jpg" alt="" style={{ width: "100%", height: "100%", borderRadius: "10px" }} />
              </a>
            </section>
            <section className='image-wrapper' style={{ width: "750px", height: "760px" }}>
              <a href="/map">
                <img src="src/assets/wind-map.png" alt="" style={{ width: "100%", height: "100%", borderRadius: "10px" }} />
              </a>
            </section>
          </div>
        </main>
        <main style={{ backgroundImage: 'url("https://images.pexels.com/photos/247763/pexels-photo-247763.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750&dpr=1")', width: "100%", height: "1000px", display: "flex", justifyContent: "center", alignItems: "center", backgroundRepeat: "no-repeat", backgroundSize: "cover"}}>
          <div style={{width: "900px", height:"400px", backgroundColor: "#fff", padding:"20px", borderRadius: "10px"}}>
            <h1>Lorem ipsum, dolor sit amet consectetur adipisicing elit. Animi molestias, omnis nostrum autem pariatur exercitationem quis necessitatibus consequatur repudiandae suscipit sapiente a nesciunt voluptatum, est veritatis eum sunt ducimus vitae.
              Lorem ipsum dolor sit amet consectetur adipisicing elit. Ex laudantium placeat ab velit facere a porro, nobis, temporibus, fugit error labore autem ea exercitationem nihil culpa. Cupiditate esse placeat veniam!
            </h1>
          </div>
        </main>
      </div>
    </>
  );
}

export default Home;
