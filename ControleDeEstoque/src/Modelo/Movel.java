
package Modelo;

/**

	A classe Movel representa informações sobre móveis.

	* @author Carlos Henrique
	* @author André João
	* @author Sunamita Vitória
	* 
	* @version 2.0
	*/

	/**

	Variável de instância privada para armazenar informações sobre o tipo de madeira.
 	*/

public class Movel extends Item {
	
	
	private String tipodemadeira;
	
	/**

	Construtor da classe Movel.
	@param nomeitem o nome do item
	@param cor a cor do item
	@param marca a marca do item
	@param modelo o modelo do item
	@param datadefabricação a data de fabricação do item
	@param valorproduto o valor do produto
	@param codigoproduto o código do produto
	@param quantidadeproduto a quantidade do produto
	@param tipodemadeira o tipo de madeira do móvel
	*/
	private Filial filial;
	public Movel(String nomeitem, String cor, String marca, String modelo, double datadefabricação, double valorproduto,
			double codigoproduto, double quantidadeproduto, String tipodemadeira) {
		super(nomeitem, cor, marca, modelo, datadefabricação, valorproduto, codigoproduto, quantidadeproduto);
		this.tipodemadeira = tipodemadeira;
	}
	
	/**

	Construtor vazio da classe Movel.
	*/
	
	public Movel() {
		
	}

	/**

	Construtor da classe Movel.

	@param nomeitem o nome do item
	@param cor a cor do item
	@param marca a marca do item
	@param modelo o modelo do item
	@param datadefabricação a data de fabricação do item
	@param valorproduto o valor do produto
	@param codigoproduto o código do produto
	@param quantidadeproduto a quantidade do produto
	*/
	
	public Movel(String nomeitem, String cor, String marca, String modelo, double datadefabricação, double valorproduto,
			int codigoproduto, int quantidadeproduto) {
		
		super();
		this.nomeitem = nomeitem;
		this.cor = cor;
		this.marca = marca;
		this.modelo = modelo;
		this.datadefabricação = datadefabricação;
		this.valorproduto = valorproduto;
		this.codigoproduto = codigoproduto;
		this.quantidadeproduto = quantidadeproduto;
		
	}

	
	/**

	Retorna o tipo de madeira do móvel.
	@return o tipo de madeira do móvel
	*/
	public String getTipodemadeira() {
	return tipodemadeira;
	}
	/**

	Define o tipo de madeira do móvel.
	@param tipodemadeira o tipo de madeira do móvel
	*/
	public void setTipodemadeira(String tipodemadeira) {
	this.tipodemadeira = tipodemadeira;
	}
	/**

	Retorna a filial relacionada ao móvel.
	@return a filial relacionada ao móvel
	*/
	public Filial getFilial() {
	return filial;
	}
	/**

	Define a filial relacionada ao móvel.
	@param filial a filial relacionada ao móvel
	*/
	public void setFilial(Filial filial) {
	this.filial = filial;
	}
	/**

	Retorna uma representação em formato de string dos dados do móvel.
	@return uma representação em formato de string dos dados do móvel
	*/
	public String toString() {
	return "Produto: " + this.getNomeitem() + "\nMarca:" + this.getMarca() + "\nCor :" + this.getCor()
	+ "\nNome específico: " + this.getModelo() + "\nData De Fabricacao: " + this.getDatadefabricação()
	+ "\nCodigo:" + this.getCodigoproduto() + "\nquantidade: " + this.getQuantidadeproduto() + "\nvalor R$"
	+ this.getValorproduto() + "\n\n";
	
	}	
	
}
