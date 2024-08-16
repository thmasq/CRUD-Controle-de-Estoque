package Modelo;


/**
 * A classe Item é uma classe abstrata que representa informações sobre um item.
 *
 * @author Carlos Henrique
 * @author André João
 * @author Sunamita Vitória
 * 
 * @version 2.0
 */

/**
 * Variáveis de instância públicas para armazenar informações sobre o item, como nome, cor, marca, modelo,
 * data de fabricação, valor do produto, código do produto e quantidade do produto.
 */			

public abstract class Item {
	
	
	protected String nomeitem;
	protected String cor;
	protected String marca;
	protected String modelo;
	protected double datadefabricação;
	protected double valorproduto;
	protected double codigoproduto;
	protected double quantidadeproduto;
	
	/**
     * Construtor para inicializar as variáveis de instância com os valores fornecidos como argumentos.
     *
     * @param nomeitem          O nome do item.
     * @param cor               A cor do item.
     * @param marca             A marca do item.
     * @param modelo            O modelo do item.
     * @param datadefabricação  A data de fabricação do item.
     * @param valorproduto      O valor do produto.
     * @param codigoproduto     O código do produto.
     * @param quantidadeproduto A quantidade do produto.
     */
	
	public Item(String nomeitem, String cor, String marca, String modelo, double datadefabricação, double valorproduto,
			double codigoproduto, double quantidadeproduto) {
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
     * Construtor vazio da classe Item.
     */
	
	public Item() {
		
	}

	/**
     * Retorna o nome do item.
     *
     * @return O nome do item.
     */
	
	public String getNomeitem() {
		return nomeitem;
	}
	
	/**
     * Define o nome do item.
     *
     * @param nomeitem O nome do item.
     */
	
	public void setNomeitem(String nomeitem) {
		this.nomeitem = nomeitem;
	}
	
	/**
     * Retorna a cor do item.
     *
     * @return A cor do item.
     */
	
	public String getCor() {
		return cor;
	}
	
	/**
     * Define a cor do item.
     *
     * @param cor A cor do item.
     */
	
	public void setCor(String cor) {
		this.cor = cor;
	}
	
	/**
     * Define a cor do item.
     *
     * @param cor A cor do item.
     */
	
	public String getMarca() {
		return marca;
	}
	
	/**
     * Retorna a marca do item.
     *
     * @return A marca do item.
     */
	
	public void setMarca(String marca) {
		this.marca = marca;
	}
	
	/**
     * Define a marca do item.
     *
     * @param marca A marca do item.
     */
	
	public String getModelo() {
		return modelo;
	}
	
	/**
     * Retorna o modelo do item.
     *
     * @return O modelo do item.
     */
	
	public void setModelo(String modelo) {
		this.modelo = modelo;
	}
	
	/**
     * Define o modelo do item.
     *
     * @param modelo O modelo do item.
     */
	
	public double getDatadefabricação() {
		return datadefabricação;
	}
	
	/**
     * Retorna a data de fabricação do item.
     *
     * @return A data de fabricação do item.
     */
	
	public void setDatadefabricação(double datadefabricação) {
		this.datadefabricação = datadefabricação;
	}
	
	/**
     * Define a data de fabricação do item.
     *
     * @param datadefabricação A data de fabricação do item.
     */
	
	public double getValorproduto() {
		return valorproduto;
	}
	
	/**
     * Retorna o valor do produto.
     *
     * @return O valor do produto.
     */
	
	public void setValorproduto(double valorproduto) {
		this.valorproduto = valorproduto;
	}
	
	/**
     * Define o valor do produto.
     *
     * @param valorproduto O valor do produto.
     */
	
	public double getCodigoproduto() {
		return codigoproduto;
	}
	
	/**
     * Retorna o código do produto.
     *
     * @return O código do produto.
     */
	
	public void setCodigoproduto(double codigoproduto) {
		this.codigoproduto = codigoproduto;
	}
	
	/**
     * Define o código do produto.
     *
     * @param codigoproduto O código do produto.
     */
	
	public double getQuantidadeproduto() {
		return quantidadeproduto;
	}
	
	/**
     * Retorna a quantidade do produto.
     *
     * @return A quantidade do produto.
     */
	
	public void setQuantidadeproduto(double quantidadeproduto) {
		this.quantidadeproduto = quantidadeproduto;
	}
	
	/**
     * Define a quantidade do produto.
     *
     * @param quantidadeproduto A quantidade do produto.
     */
	
	public String getTipodemadeira() {
		return getTipodemadeira();
	}
	
	/**
     * Retorna o tipo de madeira do item.
     * Este método deve ser implementado pelas classes que herdam de Item.
     *
     * @return O tipo de madeira do item.
     */


    /**
     * Método toString da classe Item, que retorna uma representação em string do item.
     * Este método pode ser sobrescrito nas classes que herdam de Item para fornecer uma implementação específica.
     *
     * @return Uma representação em string do item.
     */
	
	public String toString() {
		return "";
	}
	
}
